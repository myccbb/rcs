package enum

type StatusCode int64

const (
	Success      StatusCode = 0
	Fail         StatusCode = 1
	DBError      StatusCode = 2
	SystemError  StatusCode = 3
	InvalidParam StatusCode = 4
	Unauthorized StatusCode = 5
)
