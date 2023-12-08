package object

import (
	"center-server-go/enum"
	"center-server-go/internal/database"
	"center-server-go/model"
	"center-server-go/utils/response"
	"encoding/json"
	logutils "goutils/zaplog"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"gorm.io/gorm"
)

type getObjectReq struct {
	Id enum.ObjectId `json:"id" validate:"required"`
}

type getObjectRes struct {
	InternalId int64         `json:"internal_id"`
	Id         enum.ObjectId `json:"id"`
	Title      string        `json:"title"`
	Content    any           `json:"content"`
	CreateTime string        `json:"create_time"`
	UpdateTime string        `json:"update_time"`
}

func GetObject(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := getObjectReq{}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	db := database.GetConnection()
	r := model.Object{Id: req.Id}
	if err := r.GetById(db); err != nil {
		if err == gorm.ErrRecordNotFound {
			response.Success(c, nil)
			return
		}
		logger.Error("failed to get Object", zap.Error(err))
		response.Error(c, enum.DBError, err.Error())
		return
	}
	res := &getObjectRes{
		InternalId: r.InternalId,
		Id:         r.Id,
		Title:      r.Title,
		CreateTime: r.CreateTime,
		UpdateTime: r.UpdateTime,
	}
	if r.Content != "" {
		if err := json.Unmarshal([]byte(r.Content), &res.Content); err != nil {
			logger.Error("failed to unmarshal content", zap.Error(err))
			response.Error(c, enum.InvalidParam, err.Error())
			return
		}
	}
	response.Success(c, res)
}
