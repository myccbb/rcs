package tests

import (
	"center-server-go/config"
	"center-server-go/controller"
	"center-server-go/internal/database"
	"goutils/zaplog"
	"log"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
	"go.uber.org/zap"
)

func Init() (*gin.Engine, error) {
	logger := zap.Must(zaplog.ProdZapLogger())
	os.Args = []string{"test", "-cfg", "config-test.json"}
	if err := config.Init(logger); err != nil {
		log.Fatal("failed to init config", zap.Error(err))
		return nil, err
	}
	err := database.InitConnection(config.GetDBConfig(), logger)
	if err != nil {
		return nil, err
	}
	return controller.InitRouter(), nil
}

func TestRun(t *testing.T) {
	t.Logf("begin")
	router, err := Init()
	if err != nil {
		t.Fatalf("failed to init, %s", err)
		return
	}
	t.Logf("init success")
	w := httptest.NewRecorder()
	req, err := http.NewRequest(http.MethodGet, "/test-run", nil)
	if err != nil {
		t.Fatalf("failed to run test request, %s", err)
		return
	}
	router.ServeHTTP(w, req)
	assert.Equal(t, 200, w.Code)
	assert.Equal(t, "{\"code\":0,\"msg\":\"success\",\"data\":\"success\"}", w.Body.String())
}
