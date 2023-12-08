package model

import (
	"center-server-go/enum"
	"time"

	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

type ObjectType struct {
	BaseModel
	Id          enum.ObjectId     `gorm:"column:id"`
	Name        string            `gorm:"column:name"`
	Category    enum.TypeCategory `gorm:"column:category"`
	Description string            `gorm:"column:description"`
}

func (r *ObjectType) TableName() string {
	return "object_type"
}

func (r *ObjectType) Create(db *gorm.DB) error {
	if r.CreateTime == "" {
		r.CreateTime = time.Now().UTC().Format(time.RFC3339)
	}
	if r.UpdateTime == "" {
		r.UpdateTime = r.CreateTime
	}
	return db.Clauses(clause.Insert{Modifier: "OR IGNORE"}).Create(r).Error
}

func (r *ObjectType) UpdateByInternalId(db *gorm.DB) error {
	if r.UpdateTime == "" {
		r.UpdateTime = time.Now().UTC().Format(time.RFC3339)
	}
	return db.Table(r.TableName()).Where("internal_id=?", r.InternalId).Updates(map[string]any{
		"id":          r.Id,
		"name":        r.Name,
		"category":    r.Category,
		"update_time": r.UpdateTime,
	}).Error
}

func (r *ObjectType) Update(db *gorm.DB) error {
	if r.UpdateTime == "" {
		r.UpdateTime = time.Now().UTC().Format(time.RFC3339)
	}
	return db.Table(r.TableName()).Where("id=?", r.Id).Updates(map[string]any{
		"name":        r.Name,
		"category":    r.Category,
		"update_time": r.UpdateTime,
	}).Error
}

func (r *ObjectType) GetById(db *gorm.DB) error {
	return db.Where("id = ?", r.Id).First(r).Error
}

func (r *ObjectType) GetByName(db *gorm.DB) error {
	return db.Where("category=? AND name=?", r.Category, r.Name).First(r).Error
}

type ObjectTypeList struct {
	Page     int           `json:"page"`
	PageSize int           `json:"page_size"`
	Total    int64         `json:"total"`
	Results  []*ObjectType `json:"results"`
}

func (l *ObjectTypeList) TableName() string {
	return "object_type"
}

func (l *ObjectTypeList) List(db *gorm.DB, name string, category enum.TypeCategory) error {
	db = db.Table(l.TableName())
	if name != "" {
		db = db.Where("name like ?", name)
	}
	if category != "" {
		db = db.Where("category = ?", category)
	}
	err := db.Count(&l.Total).Error
	if err != nil {
		return err
	}
	return db.Limit(l.PageSize).Offset((l.Page - 1) * l.PageSize).Find(&l.Results).Error
}
