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

type getTypeReq struct {
	Id enum.ObjectId `json:"id" validate:"required"`
}

type getTypeRes struct {
	InternalId int64             `json:"internal_id"`
	Id         enum.ObjectId     `json:"id"`
	Name       string            `json:"name"`
	Category   enum.TypeCategory `json:"category"`
	CreateTime string            `json:"create_time"`
	UpdateTime string            `json:"update_time"`
}

func GetObjectType(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := getTypeReq{}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	db := database.GetConnection()
	r := model.ObjectType{Id: req.Id}
	if err := r.GetById(db); err != nil {
		logger.Error("failed to get type", zap.Error(err))
		response.Error(c, enum.DBError, err.Error())
		return
	}
	res := &getTypeRes{
		InternalId: r.InternalId,
		Id:         r.Id,
		Category:   r.Category,
		CreateTime: r.CreateTime,
		UpdateTime: r.UpdateTime,
	}
	response.Success(c, res)
}
