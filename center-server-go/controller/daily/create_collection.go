package daily

import (
	"center-server-go/enum"
	"center-server-go/internal/ctx"
	dailySvc "center-server-go/internal/daily"
	"center-server-go/internal/database"
	"center-server-go/utils/response"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

type createMatterCollectionReq struct {
	Title string `json:"title" binding:"required"`
}

type createMatterCollectionRes struct {
	Id enum.ObjectId `json:"id"`
}

func CreateCollection(ginctx *gin.Context) {
	ctx := ctx.FromGinContext(ginctx)
	logger := ctx.Logger()
	req := &createMatterCollectionReq{}
	if err := ctx.GinCtx().ShouldBindJSON(req); err != nil {
		logger.Error("parse request body failed", zap.Error(err))
		response.ErrorCtx(ctx, enum.InvalidParam, "parse request body failed")
		return
	}

	db := database.GetConnection()
	daily, err := dailySvc.GetDaily(db)
	if err != nil {
		logger.Error("get daily failed", zap.Error(err))
		response.ErrorCtx(ctx, enum.SystemError, "get daily failed")
		return
	}

	mc, err := daily.CreateMatterCollection(ctx, db, req.Title)
	if err != nil {
		logger.Error("create matter collection failed", zap.Error(err))
		response.ErrorCtx(ctx, enum.DBError, "create matter collection failed")
		return
	}
	response.SuccessCtx(ctx, createMatterCollectionRes{Id: mc.Id})
}
