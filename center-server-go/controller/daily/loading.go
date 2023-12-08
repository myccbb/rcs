package daily

import (
	"center-server-go/enum"
	"center-server-go/internal/ctx"
	"center-server-go/internal/daily"
	"center-server-go/internal/database"
	"center-server-go/model"
	"center-server-go/utils/response"
	"encoding/json"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

func Loading(ginctx *gin.Context) {
	ctx := ctx.FromGinContext(ginctx)
	logger := ctx.Logger()
	db := database.GetConnection()

	dailyRecord := model.Object{Id: daily.DailyId}
	if err := dailyRecord.GetById(db); err != nil {
		logger.Error("get daily record failed", zap.Error(err))
		response.ErrorCtx(ctx, enum.DBError, "get daily record failed")
		return
	}

	dailyContent := &daily.DailyContent{}
	if dailyRecord.Content == "" {
		dailyRecord.Content = "{}"
	}
	if err := json.Unmarshal([]byte(dailyRecord.Content), dailyContent); err != nil {
		logger.Error("unmarshal daily content failed", zap.Error(err))
		response.ErrorCtx(ctx, enum.SystemError, "unmarshal daily content failed")
		return
	}
	response.SuccessCtx(ctx, loadingRes{
		Content: dailyContent,
	})
}

type loadingRes struct {
	Content *daily.DailyContent `json:"content"`
}
