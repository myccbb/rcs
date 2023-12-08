package model

import (
	"center-server-go/enum"
	"time"

	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

type ObjectRel struct {
	BaseModel
	ParentId enum.ObjectId `gorm:"column:parent_id"`
	SubId    enum.ObjectId `gorm:"column:sub_id"`
}

func (r *ObjectRel) TableName() string {
	return "object_rel"
}

func (r *ObjectRel) Create(db *gorm.DB) error {
	if r.CreateTime == "" {
		r.CreateTime = time.Now().UTC().Format(time.RFC3339)
	}
	if r.UpdateTime == "" {
		r.UpdateTime = r.CreateTime
	}
	return db.Clauses(clause.Insert{Modifier: "OR IGNORE"}).Create(r).Error
}

func (r *ObjectRel) DeleteObjectRelationByParentIds(
	db *gorm.DB, subId enum.ObjectId, parentIds []enum.ObjectId,
) error {
	return db.Table(r.TableName()).
		Where("sub_id=? and parent_id in (?)", subId, parentIds).
		Delete(new(ObjectRel)).Error
}

func (r *ObjectRel) DeleteObjectRelationByNoParentIds(
	db *gorm.DB, subId enum.ObjectId, noParentIds []enum.ObjectId,
) error {
	db = db.Table(r.TableName()).Where("sub_id=?", subId)
	if len(noParentIds) > 0 {
		db = db.Where("parent_id not in (?)", noParentIds)
	}
	return db.Delete(new(ObjectRel)).Error
}

func (r *ObjectRel) DeleteObjectRelationBySubIds(
	db *gorm.DB, parentId enum.ObjectId, subIds []enum.ObjectId,
) error {
	return db.Table(r.TableName()).
		Where("parent_id=? and sub_id in (?)", parentId, subIds).
		Delete(new(ObjectRel)).Error
}

func (r *ObjectRel) DeleteObjectRelationByNoSubIds(
	db *gorm.DB, parentId enum.ObjectId, noSubIds []enum.ObjectId,
) error {
	db = db.Table(r.TableName()).Where("parent_id=?", parentId)
	if len(noSubIds) > 0 {
		db = db.Where("sub_id not in (?)", noSubIds)
	}
	return db.Delete(new(ObjectRel)).Error
}
