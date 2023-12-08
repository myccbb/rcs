package main

import (
	"center-server-go/config"
	"center-server-go/controller"
	"center-server-go/internal/daily"
	"center-server-go/internal/database"

	"fmt"
	"goutils/zaplog"

	"go.uber.org/zap"
)

var logger *zap.Logger

func Init() {
	var err error
	logger, err = zaplog.ProdZapLogger()
	if err != nil {
		panic("failed to init logger " + err.Error())
	}
}

func main() {
	Init()
	if err := config.Init(logger); err != nil {
		logger.Fatal("failed to init config", zap.Error(err))
		panic(err)
	}
	err := database.InitConnection(config.GetDBConfig(), logger)
	if err != nil {
		return
	}
	if err := database.InitDB(logger); err != nil {
		return
	}
	if err := daily.InitData(database.GetConnection()); err != nil {
		return
	}
	httpServerConfig := config.GetHTTPServerConfig()
	router := controller.InitRouter()

	logger.Info("server start")
	router.Run(fmt.Sprintf("%v:%v", httpServerConfig.Host, httpServerConfig.Port))
}
