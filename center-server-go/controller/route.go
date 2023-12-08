package controller

import (
	"time"

	ginzap "github.com/gin-contrib/zap"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"

	logutils "goutils/zaplog"

	"center-server-go/controller/category"
	"center-server-go/controller/daily"
	"center-server-go/controller/object"
	"center-server-go/controller/objtype"
	"center-server-go/utils/response"
)

func InitRouter() *gin.Engine {
	r := gin.New()
	r.ForwardedByClientIP = false
	logger := zap.Must(logutils.ProdZapLogger())

	r.Use(ginzap.Ginzap(logger, time.RFC3339, true))
	r.Use(ginzap.RecoveryWithZap(logger, true))
	r.Use(logutils.GinLoggerMiddleware)
	r.SetTrustedProxies([]string{"stop warn"})

	r.GET("/test-run", func(ctx *gin.Context) {
		response.Success(ctx, "success")
	})

	r.POST("/center-server/api/v1/category/list", category.ListCategory)

	r.POST("/center-server/api/v1/object-type/new", objtype.CreateObjectType)
	r.PUT("/center-server/api/v1/object-type/update", objtype.UpdateObjectType)
	r.POST("/center-server/api/v1/object-type/list", objtype.ListObjectType)
	r.POST("/center-server/api/v1/object-type/detail", objtype.GetObjectType)

	r.POST("/center-server/api/v1/object/new", object.CreateObject)
	r.PUT("/center-server/api/v1/object/update", object.UpdateObject)
	r.POST("/center-server/api/v1/object/list", object.ListObject)
	r.POST("/center-server/api/v1/object/detail", object.GetObject)

	r.POST("/center-server/api/v1/daily/loading", daily.Loading)
	r.POST("/center-server/api/v1/daily/matter_collection/detail", daily.GetCollection)

	return r
}
