package daily

import (
	"center-server-go/enum"
	"center-server-go/internal/ctx"
	"center-server-go/internal/object"
	"center-server-go/model"
	"encoding/json"

	"go.uber.org/zap"
	"gorm.io/gorm"
)

const MatterCollectionIDPrefix enum.IdPrefix = "MC"

type MatterCollectionList []*MatterCollectionRef

// func (l MatterCollectionList) FillRefObj(ctx *ctx.Ctx, db *gorm.DB) (MatterCollectionList, error) {
// 	list := MatterCollectionList{}
// 	for _, matterCollectionRef := range l {
// 		matterCollection, err := matterCollectionRef.FillRefObj(ctx, db)
// 		if err != nil {
// 			return nil, err
// 		}
// 		if matterCollection == nil {
// 			continue
// 		}
// 		list = append(list, matterCollection)
// 	}
// 	return list, nil
// }

func (l *MatterCollectionList) FindCollectionById(id enum.ObjectId) (*MatterCollectionList, int) {
	if l == nil {
		return nil, 0
	}
	for index, ref := range *l {
		if ref.Id == id {
			return l, index
		}
		if len(ref.SubCollectionList) > 0 {
			if list, pos := ref.SubCollectionList.FindCollectionById(id); list != nil {
				return list, pos
			}
		}
	}
	return nil, 0
}

type MatterCollectionRef struct {
	Id enum.ObjectId `json:"id"`

	SubCollectionList MatterCollectionList `json:"sub_collection_list,omitempty"`

	// Obj *MatterCollection `json:"-"`
}

// func (ref MatterCollectionRef) FillRefObj(ctx *ctx.Ctx, db *gorm.DB) (*MatterCollectionRef, error) {
// 	logger := ctx.Logger().With(zap.String("id", string(ref.Id)))
// 	refObj := &MatterCollection{
// 		BaseModel: object.BaseModel{
// 			Id: ref.Id,
// 		},
// 	}
// 	if err := refObj.GetById(ctx, db); err != nil {
// 		logger.Error("get matter collection ref object failed", zap.Error(err))
// 		return nil, errors.New("get matter collection ref object failed")
// 	}
// 	if refObj.Id == "" {
// 		return nil, nil
// 	}
// 	result := &MatterCollectionRef{
// 		Id: ref.Id,
// 		Obj: &MatterCollection{
// 			BaseModel: object.BaseModel{
// 				Id: ref.Id,
// 			},
// 		},
// 	}
// 	var err error
// 	result.SubMatterCollectionList, err = ref.SubMatterCollectionList.FillRefObj(ctx, db)
// 	if err != nil {
// 		logger.Error("fill sub matter collection list ref object failed", zap.Error(err))
// 		return nil, errors.New("fill sub matter collection list ref object failed")
// 	}
// 	return result, nil
// }

type MatterCollection struct {
	object.BaseModel

	MatterCollectionContent
}

func (mc *MatterCollection) GetById(ctx *ctx.Ctx, db *gorm.DB) error {
	logger := ctx.Logger().With(zap.String("id", string(mc.Id)))
	var err error
	matterCollection := model.Object{Id: mc.Id}
	if err = matterCollection.GetById(db); err != nil {
		if err == gorm.ErrRecordNotFound {
			logger.Warn("matter collection not found")
			mc.BaseModel.Id = ""
			return nil
		}
		logger.Error("get matter collection by id failed", zap.Error(err))
		return err
	}

	if mc.MatterCollectionContent.FromDBContent(matterCollection.Content); err != nil {
		logger.Error("fill matter collection content failed", zap.Error(err))
		return err
	}

	if err := mc.BaseModel.FromObjectDBRecord(&matterCollection); err != nil {
		logger.Error("fill matter collection base model failed", zap.Error(err))
		return err
	}
	return nil
}

type MatterCollectionContent struct {
	Title          string         `json:"title"`
	MatterItemList MatterItemList `json:"matter_item_list,omitempty"`
}

func (mcc *MatterCollectionContent) FromDBContent(content string) error {
	return json.Unmarshal([]byte(content), mcc)
}
