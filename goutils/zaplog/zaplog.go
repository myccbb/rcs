package zaplog

import (
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

func ProdZapLogger() (*zap.Logger, error) {
	config := zap.NewProductionConfig()
	config.EncoderConfig.EncodeTime = zapcore.ISO8601TimeEncoder
	config.DisableStacktrace = true
	logger, err := config.Build()
	if err != nil {
		return nil, err
	}
	return logger, nil
}

const ZapLoggerKey = "ZapLogger"

func GinLoggerMiddleware(c *gin.Context) {
	logger, err := GinContextLogger(c)
	if err != nil {
		return
	}

	c.Set(ZapLoggerKey, logger.With(
		zap.String("method", c.Request.Method),
		zap.String("url", c.Request.URL.RawPath),
		// TODO remote_addr从nginx转发的header取
		zap.String("remote_addr", c.Request.RemoteAddr),
	))
}

func GinContextLogger(c *gin.Context) (*zap.Logger, error) {
	raw, ok := c.Get(ZapLoggerKey)
	if !ok {
		return ProdZapLogger()
	}
	logger, ok := raw.(*zap.Logger)
	if !ok || logger == nil {
		return ProdZapLogger()
	}
	return logger, nil
}
