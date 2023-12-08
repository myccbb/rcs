package database

import (
	"center-server-go/config"
	"goutils/zaplog"
	"regexp"
	"strings"

	"go.uber.org/zap"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

var db *gorm.DB

func InitConnection(dbConfig config.DBConfig, logger *zap.Logger) (err error) {
	db, err = gorm.Open(sqlite.Open(dbConfig.Dsn()), &gorm.Config{
		Logger: zaplog.NewGormZapLogger(logger),
	})
	if err != nil {
		logger.Error("failed to open database", zap.Error(err))
		return
	}
	//db = db.Debug()
	return err
}

func GetConnection() *gorm.DB {
	return db
}

func InitDB(logger *zap.Logger) error {
	for _, item := range initSql {
		sql := regexp.MustCompile(`\s+`).ReplaceAllString(item.sql, " ")
		sql = strings.TrimSpace(sql)
		err := db.Exec(sql).Error
		if err != nil {
			logger.Error("faled to execute sql",
				zap.String("desc", item.desc),
				zap.Error(err))
			return err
		}
	}
	return nil
}

var initSql = []struct {
	sql  string
	desc string
}{
	{
		sql: `create table if not exists label
(
	internal_id INTEGER not null primary key autoincrement,
	id          TEXT    not null,
	name        TEXT    not null,
	parent_id   TEXT    not null,
	extra       TEXT    not null,
	create_time TEXT    not null,
	update_time TEXT    not null
);`,
		desc: "create table `label`",
	},
	{
		sql:  `create unique index if not exists label_unique_id on label (id);`,
		desc: "create index `label_unique_id` for `label`",
	},
	{
		sql:  `create index if not exists label_parent_id on label (parent_id);`,
		desc: "create index `label_parent_id` for `label`",
	},
	{
		sql:  `create unique index if not exists label_name on label (name);`,
		desc: "create unique index `label_name` for `label`",
	},
	{
		sql: `create table if not exists object
(
	internal_id    INTEGER not null primary key autoincrement,
	id             TEXT    not null,
	object_type_id TEXT    not null,
	title          TEXT    not null default '',
	content        TEXT    not null,
	create_time    TEXT    not null,
	update_time    TEXT    not null
);`,
		desc: "create table `object`",
	},
	{
		sql:  `create unique index if not exists object_unique_id on object (id);`,
		desc: "create unique index `object_unique_id` for `object`",
	},
	{
		sql:  `create index if not exists object_object_type_id on object (object_type_id);`,
		desc: "create index `object_object_type_id` for `object`",
	},
	{
		sql: `create table if not exists object_rel
(
	internal_id INTEGER not null primary key autoincrement,
	parent_id   TEXT    not null,
	sub_id      TEXT    not null,
	create_time TEXT    not null,
	update_time TEXT    not null
);`,
		desc: "create table `object_rel`",
	},
	{
		sql:  `create unique index if not exists object_rel_unique_parent_sub on object_rel (parent_id, sub_id);`,
		desc: "create unique index `object_rel_unique_parent_sub` for `object_rel`",
	},
	{
		sql:  `create index if not exists object_rel_parent_id on object_rel (parent_id);`,
		desc: "create index `object_rel_parent_id` for `object_rel`",
	},
	{
		sql:  `create index if not exists object_rel_sub_id on object_rel (sub_id);`,
		desc: "create index `object_rel_sub_id` for `object_rel`",
	},
	{
		sql: `create table if not exists object_label_rel
(
	internal_id INTEGER not null primary key autoincrement,
	object_id    TEXT    not null,
	label_id    TEXT    not null,
	create_time TEXT    not null
);`,
		desc: "create table `object_label_rel`",
	},
	{
		sql:  `create unique index if not exists object_label_rel_unique_object_label on object_label_rel (object_id, label_id);`,
		desc: "create unique index `object_label_rel_unique_object_label` for `object_label_rel`",
	},
	{
		sql:  `create unique index if not exists object_label_rel_label_id on object_label_rel (label_id);`,
		desc: "create unique index `object_label_rel_label_id` for `object_label_rel`",
	},
	{
		sql:  `create unique index if not exists object_label_rel_object_id on object_label_rel (object_id);`,
		desc: "create unique index `object_label_rel_object_id` for `object_label_rel`",
	},
	{
		sql: `create table if not exists object_type
(
	internal_id INTEGER not null primary key autoincrement,
	id          TEXT    not null,
	name        TEXT    not null,
	category    TEXT    not null,
	description TEXT    not null,
	create_time TEXT    not null,
	update_time TEXT    not null
);`,
		desc: "create table `object_type`",
	},
	{
		sql:  `create unique index if not exists object_type_unique_id on object_type (id);`,
		desc: "create unique index `object_type_unique_id` for `object_type`",
	},
	{
		sql:  `create unique index if not exists object_type_unique_category_name on object_type (category, name);`,
		desc: "create unique index `object_type_unique_category_name` for `object_type`",
	},
}
