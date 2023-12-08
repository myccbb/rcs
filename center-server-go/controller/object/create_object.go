package object

import (
	"center-server-go/enum"
	"center-server-go/internal/ctx"
	"center-server-go/internal/database"
	"center-server-go/model"
	"center-server-go/utils/response"
	"encoding/json"
	"time"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

type createObjectReq struct {
	Id           enum.ObjectId   `json:"id"`
	Title        string          `json:"title"`
	ObjectTypeId enum.ObjectId   `json:"object_type_id" validate:"required"`
	ParentIdList []enum.ObjectId `json:"parent_id_list"`
	SubIdList    []enum.ObjectId `json:"sub_id_list"`
	Content      any             `json:"content"`
}

func CreateObject(ginctx *gin.Context) {
	ctx := ctx.FromGinContext(ginctx)
	logger := ctx.Logger()
	req := createObjectReq{}
	var err error
	if err = ctx.GinCtx().ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.ErrorCtx(ctx, enum.InvalidParam, err.Error())
		return
	}
	if req.Id == "" {
		req.Id = enum.ObjectIdPrefix.RandomId(time.Now())
	}

	tx := database.GetConnection().Begin()
	defer tx.Rollback()
	objectType := model.ObjectType{
		Id: req.ObjectTypeId,
	}
	if err := objectType.GetById(tx); err != nil {
		logger.Error("failed to get Object type", zap.Error(err))
		response.ErrorCtx(ctx, enum.InvalidParam, "failed to get Object type")
		return
	}
	p := model.Object{
		Id:           req.Id,
		Title:        req.Title,
		ObjectTypeId: req.ObjectTypeId,
	}
	if req.Content != nil {
		content, err := json.Marshal(req.Content)
		if err != nil {
			logger.Error("failed to marshal content", zap.Error(err))
			response.ErrorCtx(ctx, enum.InvalidParam, "failed to marshal content")
			return
		}
		p.Content = string(content)
	}
	if err = p.Create(tx); err != nil {
		logger.Error("failed to create Object", zap.Error(err))
		response.ErrorCtx(ctx, enum.DBError, "failed to create Object")
		return
	}
	if len(req.ParentIdList) > 0 {
		for _, parentId := range req.ParentIdList {
			cpr := model.ObjectRel{
				ParentId: parentId,
				SubId:    req.Id,
			}
			if err := cpr.Create(tx); err != nil {
				logger.Error("failed to create parent Object relation record", zap.Error(err))
				response.ErrorCtx(ctx, enum.DBError, "failed to create parent Object relation record")
				return
			}
		}
	}
	if len(req.SubIdList) > 0 {
		for _, subId := range req.SubIdList {
			cpr := model.ObjectRel{
				ParentId: req.Id,
				SubId:    subId,
			}
			if err := cpr.Create(tx); err != nil {
				logger.Error("failed to create sub Object relation record", zap.Error(err))
				response.ErrorCtx(ctx, enum.DBError, "failed to create sub Object relation record")
				return
			}
		}
	}
	response.SuccessCtx(ctx, nil)
}
