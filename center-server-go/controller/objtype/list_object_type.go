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

type listTypeReq struct {
	Page     int               `json:"page"`
	PageSize int               `json:"page_size"`
	Name     string            `json:"name"`
	Category enum.TypeCategory `json:"category"`
}

type listTypeRes struct {
	Page     int             `json:"page"`
	PageSize int             `json:"page_size"`
	Total    int64           `json:"total"`
	Results  []*listTypeItem `json:"results"`
}

type listTypeItem struct {
	InternalId int64             `json:"internal_id"`
	Id         enum.ObjectId     `json:"id"`
	Name       string            `json:"name"`
	Category   enum.TypeCategory `json:"category"`
	CreateTime string            `json:"create_time"`
	UpdateTime string            `json:"update_time"`
}

func ListObjectType(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := listTypeReq{Page: 1, PageSize: 10}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	db := database.GetConnection()
	l := model.ObjectTypeList{
		Page:     req.Page,
		PageSize: req.PageSize,
	}
	if err := l.List(db, req.Name, req.Category); err != nil {
		logger.Error("failed to get type", zap.Error(err))
		response.Error(c, enum.DBError, err.Error())
		return
	}
	result := &listTypeRes{}
	result.fromDBList(&l)
	response.Success(c, result)
}

func (r *listTypeRes) fromDBList(l *model.ObjectTypeList) {
	r.Page = l.Page
	r.PageSize = l.PageSize
	r.Total = l.Total
	for _, item := range l.Results {
		r.Results = append(r.Results, &listTypeItem{
			InternalId: item.InternalId,
			Id:         item.Id,
			Name:       item.Name,
			Category:   item.Category,
			CreateTime: item.CreateTime,
			UpdateTime: item.UpdateTime,
		})
	}
}
