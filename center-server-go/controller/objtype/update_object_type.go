package objtype

import (
	"center-server-go/enum"
	"center-server-go/internal/database"
	"center-server-go/model"
	"center-server-go/utils/response"
	logutils "goutils/zaplog"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

type updateTypeReq struct {
	InternalId int64             `json:"internal_id"`
	Id         enum.ObjectId     `json:"id" validate:"required"`
	Name       string            `json:"name" validate:"required"`
	Category   enum.TypeCategory `json:"category" validate:"required"`
}

func UpdateObjectType(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := updateTypeReq{}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	db := database.GetConnection()
	r := model.ObjectType{
		Id:       req.Id,
		Name:     req.Name,
		Category: req.Category,
	}
	if req.InternalId > 0 {
		r.InternalId = req.InternalId
		r.UpdateByInternalId(db)
	} else {
		r.Update(db)
	}
	if err != nil {
		logger.Error("failed to update type", zap.Error(err))
		response.Error(c, enum.DBError, "failed to update type")
		return
	}
	response.Success(c, nil)
}
