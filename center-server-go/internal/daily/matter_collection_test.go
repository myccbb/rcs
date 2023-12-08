package daily_test

import (
	"testing"

	"center-server-go/enum"
	"center-server-go/internal/daily"
)

func TestMatterCollectionList_FindCollectionById(t *testing.T) {
	list1 := &daily.MatterCollectionList{
		{Id: "1"},
		{Id: "2"},
		{Id: "3"},
	}
	list10 := &daily.MatterCollectionList{
		{Id: "11"},
		{Id: "12"},
		{Id: "13"},
	}
	list20 := &daily.MatterCollectionList{
		{Id: "21"},
		{Id: "22"},
		{Id: "23"},
	}
	complexList := &daily.MatterCollectionList{
		{Id: "1", SubCollectionList: *list10},
		{Id: "2", SubCollectionList: *list20},
		{Id: "3"},
	}
	cases := []struct {
		Name       string
		Data       *daily.MatterCollectionList
		FindId     enum.ObjectId
		ExpectList *daily.MatterCollectionList
		ExpectPos  int
	}{
		{
			Name:       "empty list",
			Data:       nil,
			FindId:     "1",
			ExpectList: nil,
			ExpectPos:  0,
		},
		{
			Name:       "simple list, not found",
			Data:       list1,
			FindId:     "999",
			ExpectList: nil,
			ExpectPos:  0,
		},
		{
			Name:       "simple list, pos 1",
			Data:       list1,
			FindId:     "2",
			ExpectList: list1,
			ExpectPos:  1,
		},
		{
			Name:       "complex list, pos 1",
			Data:       complexList,
			FindId:     "2",
			ExpectList: complexList,
			ExpectPos:  1,
		},
		{
			Name:       "complex list, sub list 1, pos 1",
			Data:       complexList,
			FindId:     "22",
			ExpectList: &(*complexList)[1].SubCollectionList,
			ExpectPos:  1,
		},
	}
	for _, c := range cases {
		t.Run(c.Name, func(t *testing.T) {
			list, pos := c.Data.FindCollectionById(c.FindId)
			if list != c.ExpectList {
				t.Errorf("expect list %p, got %p", c.ExpectList, list)
			}
			if pos != c.ExpectPos {
				t.Errorf("expect pos %d, got %d", c.ExpectPos, pos)
			}
		})
	}
}
