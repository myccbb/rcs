package enum

import (
	"math/rand"
	"time"
)

type ObjectId string

type IdPrefix string

const (
	ObjectIdPrefix IdPrefix = "OBJ"
	TypeIdPrefix   IdPrefix = "TYPE"
	LabelIdPrefix  IdPrefix = "LABEL"
)

func (c IdPrefix) RandomId(now time.Time) ObjectId {
	return ObjectId(string(c) + "-" + randomId(now))
}

func randomId(now time.Time) string {
	return now.Format("060102-150405") + "-" + randomString(6)
}

func randomString(n int) string {
	var letterRunes = []rune("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")
	b := make([]rune, n)
	for i := range b {
		b[i] = letterRunes[rand.Intn(len(letterRunes))]
	}
	return string(b)
}
