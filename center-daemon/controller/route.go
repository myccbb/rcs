package controller

import (
	"center-daemon/controller/source"
	"goutils/httpframework"
	"net/http"
)

func InitRouter() *httpframework.Router {
	router := httpframework.NewRouter("default")

	router.Add(http.MethodPost, "/center-daemon/api/v1/source/push", source.Push)

	return router
}
