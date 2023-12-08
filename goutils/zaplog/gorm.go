package zaplog

import (
	"context"
	"fmt"
	"time"

	"go.uber.org/zap"
	gormLogger "gorm.io/gorm/logger"
)

type GormZapLogger struct {
	logger *zap.Logger
}

func NewGormZapLogger(logger *zap.Logger) *GormZapLogger {
	logger = logger.WithOptions(zap.AddCallerSkip(3))
	return &GormZapLogger{logger: logger.WithOptions()}
}

func (l GormZapLogger) LogMode(level gormLogger.LogLevel) gormLogger.Interface {
	return &GormZapLogger{logger: l.logger.WithOptions()}
}

func (l *GormZapLogger) Info(ctx context.Context, msg string, params ...any) {
	l.logger.Info(fmt.Sprintf(msg, params...))
}

func (l *GormZapLogger) Warn(ctx context.Context, msg string, params ...any) {
	l.logger.Warn(fmt.Sprintf(msg, params...))
}

func (l *GormZapLogger) Error(ctx context.Context, msg string, params ...any) {
	l.logger.Error(fmt.Sprintf(msg, params...))
}

func (l *GormZapLogger) Trace(
	ctx context.Context,
	begin time.Time,
	fc func() (sql string, rowsAffected int64),
	err error,
) {
	elapsed := time.Since(begin)
	sql, rows := fc()
	l.logger.Info("trace log", zap.Duration("elapsed", elapsed),
		zap.String("sql", sql), zap.Int64("rows", rows), zap.Error(err))
}
