package response

import (
	"center-server-go/enum"
	"center-server-go/internal/ctx"
	"net/http"

	"github.com/gin-gonic/gin"
)

type res struct {
	Code enum.StatusCode `json:"code"`
	Msg  string          `json:"msg"`
	Data any             `json:"data,omitempty"`
}

func Success(ctx *gin.Context, data any) {
	resData := res{
		Code: 0,
		Msg:  "success",
		Data: data,
	}
	ctx.JSON(http.StatusOK, resData)
}

func SuccessCtx(ctx *ctx.Ctx, data any) {
	resData := res{
		Code: 0,
		Msg:  "success",
		Data: data,
	}
	ctx.GinCtx().JSON(http.StatusOK, resData)
}

func Error(ctx *gin.Context, code enum.StatusCode, msg string) {
	resData := res{
		Code: code,
		Msg:  msg,
	}
	ctx.JSON(http.StatusOK, resData)
}

func ErrorCtx(ctx *ctx.Ctx, code enum.StatusCode, msg string) {
	resData := res{
		Code: code,
		Msg:  msg,
	}
	ctx.GinCtx().JSON(http.StatusOK, resData)
}

func AbortWithError(ctx *gin.Context, code enum.StatusCode, msg string) {
	resData := res{
		Code: code,
		Msg:  msg,
	}
	ctx.JSON(http.StatusOK, resData)
	ctx.Abort()
}

func AbortWithErrorCtx(ctx *ctx.Ctx, code enum.StatusCode, msg string) {
	resData := res{
		Code: code,
		Msg:  msg,
	}
	ctx.GinCtx().JSON(http.StatusOK, resData)
	ctx.GinCtx().Abort()
}
