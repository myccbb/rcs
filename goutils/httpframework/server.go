package httpframework

import (
	"go/ast"
	"go/doc"
	"go/parser"
	"go/token"
	"goutils/zaplog"
	"net/http"
	"path/filepath"
	"reflect"
	"runtime"
	"strconv"
	"strings"
	"time"

	"go.uber.org/zap"
)

type Server struct {
	host   string
	port   uint16
	router *Router
	logger *zap.Logger
}

func NewServer(host string, port uint16, routerInitilizer func(*zap.Logger) (*Router, error)) (*Server, error) {
	logger, err := zaplog.ProdZapLogger()
	if err != nil {
		return nil, err
	}
	router, err := routerInitilizer(logger)
	if err != nil {
		return nil, err
	}

	funcDoc, err := getFuncDoc(routerInitilizer)
	if err != nil {
		return nil, err
	}
	logger.Info("router initilizer", zap.String("doc", funcDoc))
	logger.Info("parsed router doc", zap.Any("router", ParseRouterDoc(funcDoc)))

	return &Server{
		host:   host,
		port:   port,
		router: router,
		logger: logger,
	}, nil
}

func getFuncDoc(funcObj interface{}) (string, error) {
	funcInfo := runtime.FuncForPC(reflect.ValueOf(funcObj).Pointer())
	file, _ := funcInfo.FileLine(0)
	initRouterAst, err := parser.ParseFile(token.NewFileSet(), file, nil, parser.ParseComments)
	if err != nil {
		return "", err
	}
	pkg := &ast.Package{
		Name: "Swagger",
		Files: map[string]*ast.File{
			file: initRouterAst,
		},
	}
	importPath, err := filepath.Abs("/")
	if err != nil {
		return "", err
	}
	myDoc := doc.New(pkg, importPath, doc.AllDecls)
	for _, funcDocObj := range myDoc.Funcs {
		// logger.Info("func name", zap.String("func", funcp.Name))
		// logger.Info("func doc", zap.String("doc", funcp.Doc))
		if funcDocObj.Name == getFuncName(funcInfo.Name()) {
			return funcDocObj.Doc, nil
		}
	}
	return "", nil
}

func parseStruct(typ reflect.Type) {}

func getFuncName(fullFuncName string) string {
	parts := strings.Split(fullFuncName, ".")
	return parts[len(parts)-1]
}

func (s Server) Start() error {
	hs := http.Server{
		Addr:         s.host + ":" + strconv.Itoa(int(s.port)),
		Handler:      s,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 10 * time.Second,
	}
	return hs.ListenAndServe()
}

func (s Server) ServeHTTP(resWriter http.ResponseWriter, req *http.Request) {
	method := req.Method
	path := strings.Trim(req.URL.Path, "/")
	s.logger.Info("request", zap.String("method", method), zap.String("path", path))
	r, ok := s.router.get(method, path)
	if !ok {
		resWriter.WriteHeader(http.StatusNotFound)
		return
	}
	logger := s.logger.With(
		zap.String("method", method),
		zap.String("url", path),
		// TODO remote_addr从nginx转发的header取
		zap.String("remote_addr", req.RemoteAddr),
	)
	ctx := &Context{
		req:       req,
		resWriter: resWriter,
		Logger:    logger,
	}
	r.handle(ctx)
}
