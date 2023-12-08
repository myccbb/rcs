package ctx

import (
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"

	logutils "goutils/zaplog"
)

type Ctx struct {
	logger *zap.Logger
	ginctx *gin.Context
}

func FromGinContext(ginctx *gin.Context) *Ctx {
	c := &Ctx{}
	if ginctx == nil {
		panic("ginctx is nil")
	}
	c.logger = zap.Must(logutils.GinContextLogger(ginctx))
	c.ginctx = ginctx
	return c
}

type LoggerOption struct {
	Named string
}

func (c *Ctx) Logger() *zap.Logger {
	return c.logger
}

func (c *Ctx) GinCtx() *gin.Context {
	return c.ginctx
}
