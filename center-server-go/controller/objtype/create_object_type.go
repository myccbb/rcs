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

type createTypeReq struct {
	Id          enum.ObjectId     `json:"id" validate:"required"`
	Name        string            `json:"name" validate:"required"`
	Category    enum.TypeCategory `json:"category" validate:"required"`
	Description string            `json:"description"`
}

func CreateObjectType(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := createTypeReq{}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	db := database.GetConnection()
	r := model.ObjectType{
		Id:          req.Id,
		Name:        req.Name,
		Category:    req.Category,
		Description: req.Description,
	}
	if err = r.Create(db); err != nil {
		logger.Error("failed to create type", zap.Error(err))
		response.Error(c, enum.DBError, "failed to create type")
		return
	}
	response.Success(c, nil)
}
