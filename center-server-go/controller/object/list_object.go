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

type listObjectReq struct {
	Page     int `query:"page"`
	PageSize int `query:"page_size"`
}

type listObjectRes struct {
	Page     int               `json:"page"`
	PageSize int               `json:"page_size"`
	Total    int64             `json:"total"`
	Results  []*listObjectItem `json:"results"`
}

type listObjectItem struct {
	InternalId int64          `json:"internal_id"`
	Id         enum.ObjectId  `json:"id"`
	Title      string         `json:"title"`
	Content    map[string]any `json:"content"`
	CreateTime string         `json:"create_time"`
	UpdateTime string         `json:"update_time"`
}

func ListObject(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := listObjectReq{}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	db := database.GetConnection()
	l := model.ObjectList{}
	if err := l.List(db); err != nil {
		if err == gorm.ErrRecordNotFound {
			response.Success(c, nil)
			return
		}
		logger.Error("failed to get Object", zap.Error(err))
		response.Error(c, enum.DBError, err.Error())
		return
	}
	result := &listObjectRes{}
	if err := result.fromDBList(&l); err != nil {
		response.Error(c, enum.SystemError, "failed to parse Object content")
		return
	}
	response.Success(c, result)
}

func (r *listObjectRes) fromDBList(l *model.ObjectList) error {
	r.Page = l.Page
	r.PageSize = l.PageSize
	r.Total = l.Total
	for _, record := range l.Results {
		Object := &listObjectItem{
			InternalId: record.InternalId,
			Id:         record.Id,
			Title:      record.Title,
			CreateTime: record.CreateTime,
			UpdateTime: record.UpdateTime,
		}
		if record.Content != "" {
			if err := json.Unmarshal([]byte(record.Content), &Object.Content); err != nil {
				return err
			}
		}
		r.Results = append(r.Results, &listObjectItem{})
	}
	return nil
}
