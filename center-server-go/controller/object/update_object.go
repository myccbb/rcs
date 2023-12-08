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
)

type updateObjectReq struct {
	InternalId *int64         `json:"internal_id"`
	Id         *enum.ObjectId `json:"id"`
	Title      *string        `json:"title"`

	ObjectTypeId       *enum.ObjectId `json:"object_type_id"`
	VerifyObjectTypeId bool           `json:"verify_object_type_id"`

	Content *json.RawMessage `json:"content"`

	AllParentIdList *[]enum.ObjectId `json:"all_parent_id_list"`
	AddParentIdList []enum.ObjectId  `json:"add_parent_id_list"`
	DelParentIdList []enum.ObjectId  `json:"del_parent_id_list"`

	AllSubIdList *[]enum.ObjectId `json:"all_sub_id_list"`
	AddSubIdList []enum.ObjectId  `json:"add_sub_id_list"`
	DelSubIdList []enum.ObjectId  `json:"del_sub_id_list"`
}

func UpdateObject(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := updateObjectReq{}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	db := database.GetConnection()
	curObject := model.Object{}
	if req.InternalId != nil {
		curObject.InternalId = *req.InternalId
		if err := curObject.GetByInternalId(db); err != nil {
			response.Error(c, enum.InvalidParam, "invalid internal_id")
			return
		}
	} else if req.Id != nil {
		curObject.Id = *req.Id
		if err := curObject.GetById(db); err != nil {
			response.Error(c, enum.InvalidParam, "invalid id")
			return
		}
	} else {
		response.Error(c, enum.InvalidParam, "missing id")
		return
	}
	tx := db.Begin()
	defer tx.Rollback()
	updates := map[string]interface{}{}
	if req.Id != nil && req.InternalId != nil {
		updates["internal_id"] = *req.InternalId
	}
	if req.Title != nil {
		updates["title"] = *req.Title
	}
	if req.ObjectTypeId != nil {
		if req.VerifyObjectTypeId {
			objectType := model.ObjectType{
				Id: *req.ObjectTypeId,
			}
			if err := objectType.GetById(tx); err != nil {
				logger.Error("failed to get Object type", zap.Error(err))
				response.Error(c, enum.InvalidParam, "failed to get Object type")
				return
			}
		}
		updates["object_type_id"] = *req.ObjectTypeId
	}
	if req.Content != nil {
		contentBytes, err := json.Marshal(req.Content)
		if err != nil {
			logger.Error("failed to marshal content", zap.Error(err))
			response.Error(c, enum.InvalidParam, "failed to marshal content")
			return
		}
		updates["content"] = string(contentBytes)
	}
	Object := model.Object{}
	if req.InternalId != nil {
		Object.InternalId = *req.InternalId
		err = Object.UpdateByInternalId(tx, updates)
	} else if req.Id != nil {
		Object.Id = *req.Id
		err = Object.Update(tx, updates)
	}
	if err != nil {
		logger.Error("failed to update type", zap.Error(err))
		response.Error(c, enum.DBError, "failed to update type")
		return
	}
	if len(req.AddParentIdList) > 0 {
		for _, parentId := range req.AddParentIdList {
			rel := model.ObjectRel{
				ParentId: parentId,
				SubId:    Object.Id,
			}
			if err := rel.Create(tx); err != nil {
				logger.Error("failed to create parent Object relation", zap.Error(err))
				response.Error(c, enum.DBError, "failed to create parent Object relation")
				return
			}
		}
	}
	if len(req.DelParentIdList) > 0 {
		rel := model.ObjectRel{}
		if err := rel.DeleteObjectRelationByParentIds(tx, Object.Id, req.DelParentIdList); err != nil {
			logger.Error("failed to delete parent Object relation", zap.Error(err))
			response.Error(c, enum.DBError, "failed to delete parent Object relation")
			return
		}
	}
	if req.AllParentIdList != nil {
		rel := model.ObjectRel{}
		if err := rel.DeleteObjectRelationByNoParentIds(tx, Object.Id, *req.AllParentIdList); err != nil {
			logger.Error("failed to delete all parent Object relation",
				zap.String("Object id", string(Object.Id)), zap.Error(err))
			response.Error(c, enum.DBError, "failed to delete all parent Object relation")
			return
		}
		for _, parentId := range *req.AllParentIdList {
			rel := model.ObjectRel{
				ParentId: parentId,
				SubId:    Object.Id,
			}
			if err := rel.Create(tx); err != nil {
				logger.Error("failed to create parent Object relation", zap.Error(err))
				response.Error(c, enum.DBError, "failed to create parent Object relation")
				return
			}
		}
	}

	if len(req.AddSubIdList) > 0 {
		for _, subId := range req.AddSubIdList {
			rel := model.ObjectRel{
				ParentId: Object.Id,
				SubId:    subId,
			}
			if err := rel.Create(tx); err != nil {
				logger.Error("failed to create sub Object relation", zap.Error(err))
				response.Error(c, enum.DBError, "failed to create sub Object relation")
				return
			}
		}
	}
	if len(req.DelSubIdList) > 0 {
		rel := model.ObjectRel{}
		if err := rel.DeleteObjectRelationBySubIds(tx, Object.Id, req.DelSubIdList); err != nil {
			logger.Error("failed to delete sub Object relation", zap.Error(err))
			response.Error(c, enum.DBError, "failed to delete sub Object relation")
			return
		}
	}
	if req.AllSubIdList != nil {
		rel := model.ObjectRel{}
		if err := rel.DeleteObjectRelationByNoSubIds(tx, Object.Id, *req.AllSubIdList); err != nil {
			logger.Error("failed to delete all sub Object relation",
				zap.String("Object id", string(Object.Id)), zap.Error(err))
			response.Error(c, enum.DBError, "failed to delete all sub Object relation")
			return
		}
		for _, subId := range *req.AllSubIdList {
			rel := model.ObjectRel{
				SubId:    subId,
				ParentId: Object.Id,
			}
			if err := rel.Create(tx); err != nil {
				logger.Error("failed to create sub Object relation", zap.Error(err))
				response.Error(c, enum.DBError, "failed to create sub Object relation")
				return
			}
		}
	}
	if err := tx.Commit().Error; err != nil {
		logger.Error("failed to commit changes", zap.Error(err))
		response.Error(c, enum.DBError, "failed to commit changes")
		return
	}
	response.Success(c, nil)
}
