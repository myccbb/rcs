package config

import (
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"os"

	"go.uber.org/zap"
)

// Content config
type config struct {
	HTTP HTTPServerConfig `json:"http_server"`
	DB   DBConfig         `json:"db"`
}

// Init init
func (c config) Init() {
}

// ConfigPath config path
var ConfigPath string

// Cfg config
var Cfg config

// GetHTTPServerConfig get http server config
func GetHTTPServerConfig() HTTPServerConfig {
	return Cfg.HTTP
}

// GetDBConfig get database config
func GetDBConfig() DBConfig {
	return Cfg.DB
}

// Init init
func Init(log *zap.Logger) error {
	flag.StringVar(&ConfigPath, "cfg", "config-dev.json", "-cfg <filename>")
	flag.Parse()

	log.Info("config path", zap.String("fullpath", ConfigPath))

	f, err := os.Open(ConfigPath)
	if err != nil {
		return err
	}
	raw, err := io.ReadAll(f)
	if err != nil {
		return err
	}

	err = json.Unmarshal(raw, &Cfg)
	if err != nil {
		return err
	}

	log.Info("config", zap.String("http config", Cfg.HTTP.String()))
	// log.Info("config", zap.String("database config", Cfg.DB.String()))

	return nil
}

// HTTPServerConfig http server config
type HTTPServerConfig struct {
	Host string `json:"host"`
	Port uint16 `json:"port"`
}

// String string
func (c HTTPServerConfig) String() string {
	return fmt.Sprintf("%s:%d", c.Host, c.Port)
}

// DBConfig mysql config
type DBConfig struct {
	Host     string `json:"host"`
	Port     int    `json:"port"`
	Username string `json:"username"`
	Password string `json:"password"`
	DBName   string `json:"db_name"`
}

//// String string
//func (c DBConfig) String() string {
//	return fmt.Sprintf("%s@%s:%d/%s", c.Username, c.Host, c.Port, c.DBName)
//}

// Dsn dsn
func (c DBConfig) Dsn() string {
	//// checkout dsn options at https://github.com/go-sql-driver/mysql
	//dsn := fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=true&loc=UTC",
	//	c.Username, c.Password, c.Host, c.Port, c.DBName)
	return c.DBName
}

//type SqliteConfig struct {
//	DBName   string `json:"db_name"`
//	Username string `json:"username"`
//	Password string `json:"password"`
//}
//
//func (c SqliteConfig) String() string {
//	return fmt.Sprintf("%s@%s", c.Username, c.DBName)
//}
//
//type MongoConfig struct {
//	Host     string `json:"host"`
//	Port     int    `json:"port"`
//	Username string `json:"username"`
//	Password string `json:"password"`
//	DBName   string `json:"db_name"`
//}
//
//// String string
//func (c MongoConfig) String() string {
//	return fmt.Sprintf("%s@%s:%d/%s", c.Username, c.Host, c.Port, c.DBName)
//}
//
//// Dsn dsn
//func (c MongoConfig) Dsn() string {
//	// checkout dsn options at https://docs.mongodb.com/manual/reference/connection-string/
//	// mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]
//	dsn := fmt.Sprintf("mongodb://%s:%s@%s:%d/%s?authSource=%s", c.Username, c.Password, c.Host, c.Port, c.DBName, c.DBName)
//	return dsn
//}
