package httpframework

import (
	"errors"
	"strings"
	"sync"
)

type innerRouter struct {
	sync.RWMutex
	routes map[string]*routeItem
}

type Router struct {
	name        string
	inner       *innerRouter
	handlerList []Handler
}

type routeItem struct {
	method  string
	path    string
	handler Handler

	handlerList []Handler
}

type AbortError struct {
	err error
}

func (e AbortError) Error() string {
	return "AbortError"
}

func (r routeItem) handle(c *Context) {
	for _, handler := range r.handlerList {
		err := handler(c)
		abortError := &AbortError{}
		if err != nil && errors.As(err, abortError) {
			return
		}
	}
}

type Handler func(c *Context) error

func NewRouter(name string) *Router {
	return &Router{
		name: name,
		inner: &innerRouter{
			routes: make(map[string]*routeItem),
		},
	}
}

func (r *Router) New(name string) *Router {
	router := &Router{
		name:        name,
		inner:       r.inner,
		handlerList: make([]Handler, len(r.handlerList)),
	}
	copy(router.handlerList, r.handlerList)
	return router
}

func (r *Router) Use(f Handler) {
	r.handlerList = append(r.handlerList, f)
}

func (r *Router) Add(method string, path string, handler Handler) {
	r.inner.Lock()
	defer r.inner.Unlock()
	path = strings.Trim(path, "/")
	route := &routeItem{
		method, path, handler, make([]Handler, len(r.handlerList)),
	}
	copy(route.handlerList, r.handlerList)
	route.handlerList = append(route.handlerList, handler)
	r.inner.routes[method+"|"+path] = route
}

func (r *Router) get(method string, path string) (*routeItem, bool) {
	r.inner.RLock()
	defer r.inner.RUnlock()
	path = strings.Trim(path, "/")
	if route, ok := r.inner.routes[method+"|"+path]; ok {
		return route, true
	}
	return nil, false
}
