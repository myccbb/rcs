package model

type BaseModel struct {
	InternalId int64  `gorm:"column:internal_id;primaryKey"`
	CreateTime string `gorm:"column:create_time"`
	UpdateTime string `gorm:"column:update_time"`
}

type CreateOption struct {
	IgnoreExist bool
}
