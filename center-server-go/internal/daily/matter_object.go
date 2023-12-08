package daily

import (
	"center-server-go/enum"
	"center-server-go/internal/object"
)

type UpdateMatter struct {
	Done    *bool   `json:"done"`
	Content *string `json:"content"`
}

type MatterItemList []*MatterRef

type MatterRef struct {
	Id enum.ObjectId `json:"id"`
}

type MatterItem struct {
	object.BaseModel

	MatterItemContent
}

type MatterItemContent struct {
	Done    bool   `json:"done"`
	Content string `json:"content,omitempty"`

	SubMatterItemList MatterItemList `json:"sub_matter_item_list,omitempty"`
}
