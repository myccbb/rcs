package daily

import (
	"center-server-go/enum"
	"center-server-go/internal/ctx"
	"center-server-go/internal/object"
	"center-server-go/model"
	"encoding/json"
	"errors"
	"time"

	"go.uber.org/zap"
	"gorm.io/gorm"
)

type Daily struct {
	object.BaseModel

	DailyContent

	dbRecord *model.Object
}

func GetDaily(db *gorm.DB) (*Daily, error) {
	dbRecord := model.Object{Id: DailyId}
	if err := dbRecord.GetById(db); err != nil {
		return nil, err
	}
	if dbRecord.InternalId == 0 {
		return nil, errors.New("daily record not found")
	}
	daily := &Daily{}
	if err := daily.FromDBRecord(&dbRecord); err != nil {
		return nil, err
	}
	return daily, nil
}

func (d *Daily) FromDBRecord(r *model.Object) error {
	d.dbRecord = r
	if err := d.BaseModel.FromObjectDBRecord(r); err != nil {
		return err
	}
	if err := json.Unmarshal([]byte(r.Content), &d.DailyContent); err != nil {
		return err
	}
	return nil
}

func (d *Daily) CreateMatterCollection(ctx *ctx.Ctx, db *gorm.DB, title string) (*model.Object, error) {
	logger := ctx.Logger()
	tx := db.Begin()
	defer tx.Rollback()
	mc := &model.Object{
		Id:           MatterCollectionIDPrefix.RandomId(time.Now()),
		Title:        title,
		ObjectTypeId: MatterCollectionObjectType,
	}
	if err := mc.Create(tx); err != nil {
		logger.Error("create matter collection failed", zap.Error(err))
		return nil, errors.New("create matter collection failed")
	}

	// create to daily
	d.DailyContent.AddCollection(ctx, mc.Id)
	if err := d.UpdateContent(ctx, tx); err != nil {
		logger.Error("update daily content failed", zap.Error(err))
		return nil, errors.New("update daily content failed")
	}

	if err := tx.Commit().Error; err != nil {
		logger.Error("commit transaction failed", zap.Error(err))
		return nil, errors.New("commit transaction failed")
	}
	return mc, nil
}

func (d *Daily) UpdateContent(ctx *ctx.Ctx, db *gorm.DB) error {
	logger := ctx.Logger()
	contentBytes, err := json.Marshal(d.DailyContent)
	if err != nil {
		logger.Error("marshal daily content failed", zap.Error(err))
		return errors.New("marshal daily content failed")
	}
	if err := d.dbRecord.Update(db, map[string]interface{}{
		"content": string(contentBytes),
	}); err != nil {
		logger.Error("update daily content failed", zap.Error(err))
		return errors.New("update daily content failed")
	}
	return nil
}

type DailyContent struct {
	CollectionList MatterCollectionList `json:"collection_list"`

	RelatedCollectionIdList []enum.ObjectId `json:"related_collection_id_list"`
}

func (c *DailyContent) AddCollection(ctx *ctx.Ctx, collectionId enum.ObjectId) {
	c.CollectionList = append(c.CollectionList, &MatterCollectionRef{
		Id: collectionId,
	})
	c.RelatedCollectionIdList = append(c.RelatedCollectionIdList, collectionId)
}

// func (c DailyContent) FillRef(ctx *ctx.Ctx, db *gorm.DB) (*DailyContent, error) {
// 	matterCollectionList, err := c.CollectionList.FillRefObj(ctx, db)
// 	if err != nil {
// 		return nil, err
// 	}
// 	return &DailyContent{
// 		CollectionList: matterCollectionList,
// 	}, nil
// }

const (
	DailyId enum.ObjectId = "daily"
)

const (
	DailyObjectType            enum.ObjectId = "daily"
	MatterItemObjectType       enum.ObjectId = "matter_item"
	MatterCollectionObjectType enum.ObjectId = "matter_collection"
)

func InitData(db *gorm.DB) error {
	dailyObjectType := model.ObjectType{
		Id:       DailyObjectType,
		Name:     "daily",
		Category: enum.TypeCategoryCommon,
	}
	if err := dailyObjectType.Create(db); err != nil {
		return err
	}
	matterItemObjectType := model.ObjectType{
		Id:       MatterItemObjectType,
		Name:     "matter item",
		Category: enum.TypeCategoryCommon,
	}
	if err := matterItemObjectType.Create(db); err != nil {
		return err
	}
	matterCollectionObjectType := model.ObjectType{
		Id:       MatterCollectionObjectType,
		Name:     "matter collection",
		Category: enum.TypeCategoryCommon,
	}
	if err := matterCollectionObjectType.Create(db); err != nil {
		return err
	}

	dailyCollection := model.Object{
		Id:           DailyId,
		Title:        "daily",
		ObjectTypeId: DailyObjectType,
		Content:      `{"title":"daily"}`,
	}
	if err := dailyCollection.Create(db, model.CreateOption{
		IgnoreExist: true,
	}); err != nil {
		return err
	}
	return nil
}
