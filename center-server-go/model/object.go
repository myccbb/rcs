package model

import (
	"center-server-go/enum"
	"time"

	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

type Object struct {
	BaseModel
	Id           enum.ObjectId `gorm:"column:id"`
	ObjectTypeId enum.ObjectId `gorm:"column:object_type_id"`
	Title        string        `gorm:"column:title"`
	Content      string        `gorm:"column:content"`
}

func (r *Object) TableName() string {
	return "object"
}

func (r *Object) GetByInternalId(db *gorm.DB) error {
	return db.Where("internal_id = ?", r.InternalId).First(r).Error
}

func (r *Object) GetById(db *gorm.DB) error {
	return db.Where("id = ?", r.Id).First(r).Error
}

func (r *Object) Create(db *gorm.DB, options ...CreateOption) error {
	if r.CreateTime == "" {
		r.CreateTime = time.Now().UTC().Format(time.RFC3339)
	}
	if r.UpdateTime == "" {
		r.UpdateTime = r.CreateTime
	}
	if len(options) > 0 {
		option := options[0]
		if option.IgnoreExist {
			db = db.Clauses(clause.Insert{Modifier: "OR IGNORE"})
		}
	}
	return db.Create(r).Error
}

func (r *Object) Update(db *gorm.DB, updates map[string]interface{}) error {
	if _, ok := updates["update_time"]; !ok {
		updates["update_time"] = time.Now().UTC().Format(time.RFC3339)
	}
	delete(updates, "internal_id")
	return db.Table(r.TableName()).Where("id=?", r.Id).Updates(updates).Error
}

func (r *Object) UpdateByInternalId(db *gorm.DB, updates map[string]interface{}) error {
	if _, ok := updates["update_time"]; !ok {
		updates["update_time"] = time.Now().UTC().Format(time.RFC3339)
	}
	delete(updates, "internal_id")
	return db.Table(r.TableName()).Where("internal_id=?", r.InternalId).Updates(updates).Error
}

type ObjectList struct {
	Page     int       `json:"page"`
	PageSize int       `json:"page_size"`
	Total    int64     `json:"total"`
	Results  []*Object `json:"results"`
}

func (l *ObjectList) TableName() string {
	return "object"
}

func (l *ObjectList) List(db *gorm.DB) error {
	db = db.Table(l.TableName())
	err := db.Count(&l.Total).Error
	if err != nil {
		return err
	}
	return db.Limit(l.PageSize).Offset((l.Page - 1) * l.PageSize).Find(&l.Results).Error
}
