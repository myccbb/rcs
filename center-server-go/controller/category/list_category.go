package category

import (
	"center-server-go/utils/response"

	"github.com/gin-gonic/gin"
)

type listCategoryRes struct {
	Results []listCategoryItem `json:"results"`
}

type listCategoryItem struct {
	Name string `json:"name"`
}

func ListCategory(c *gin.Context) {
	result := &listCategoryRes{
		Results: []listCategoryItem{
			{
				Name: "object",
			},
		},
	}
	response.Success(c, result)
}
