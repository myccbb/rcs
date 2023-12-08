package daily

import (
	logutils "goutils/zaplog"

	"center-server-go/enum"
	"center-server-go/utils/response"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

type getCollectionReq struct {
	Id enum.ObjectId `json:"id"`
}

type getCollectionRes struct {
	Id               enum.ObjectId   `json:"id"`
	Title            string          `json:"title"`
	MatterObjectList []*MatterObject `json:"matter_object_list"`
}

func GetCollection(c *gin.Context) {
	logger := zap.Must(logutils.GinContextLogger(c))
	req := getCollectionReq{}
	var err error
	if err = c.ShouldBindJSON(&req); err != nil {
		logger.Error("failed to get request body", zap.Error(err))
		response.Error(c, enum.InvalidParam, err.Error())
		return
	}
	// db := database.GetConnection()
}

type MatterObject struct {
	Id string `json:"id"`
}
