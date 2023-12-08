package object

import (
	"center-server-go/enum"
	"center-server-go/model"
	"fmt"
	"time"
)

type BaseModel struct {
	InternalId   int64         `json:"column:internal_id"`
	Id           enum.ObjectId `json:"column:id"`
	ObjectTypeId enum.ObjectId `json:"column:object_type_id"`
	CreateTime   time.Time     `json:"column:create_time"`
	UpdateTime   time.Time     `json:"column:update_time"`
}

func (m *BaseModel) FromObjectDBRecord(record *model.Object) error {
	m.InternalId = record.InternalId
	m.Id = record.Id
	m.ObjectTypeId = record.ObjectTypeId
	var err error
	m.CreateTime, err = time.Parse(time.RFC3339, record.CreateTime)
	if err != nil {
		return fmt.Errorf("parse create_time failed: %w", err)
	}
	m.UpdateTime, err = time.Parse(time.RFC3339, record.UpdateTime)
	if err != nil {
		return fmt.Errorf("parse update_time failed: %w", err)
	}
	return nil
}
