package httpframework

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"reflect"
	"strconv"
	"strings"

	"github.com/go-playground/validator/v10"
	"go.uber.org/zap"
)

type Context struct {
	req       *http.Request
	resWriter http.ResponseWriter
	Logger    *zap.Logger
}

type FieldInfo struct {
	index int
	name  string
	tag   string
	kind  reflect.Kind
}

func newFieldInfo(index int, name, tag string, kind reflect.Kind) *FieldInfo {
	return &FieldInfo{index: index, name: name, tag: tag, kind: kind}
}

type ParseValueError struct {
	value string
	FieldInfo
}

func NewParseFieldError(fieldInfo *FieldInfo, value string) *ParseValueError {
	return &ParseValueError{
		FieldInfo: *fieldInfo,
		value:     value,
	}
}

func (e *ParseValueError) Error() string {
	return fmt.Sprintf(`failed to parse value "%s" to type %s, field name %s, tag %s`,
		e.value, e.kind, e.name, e.tag)
}

type SetFieldError struct {
	FieldInfo
}

func NewSetFieldError(fieldInfo *FieldInfo) *SetFieldError {
	return &SetFieldError{
		FieldInfo: *fieldInfo,
	}
}

func (e *SetFieldError) Error() string {
	return fmt.Sprintf("cannot set field, index %d, field name %s, tag %s",
		e.index, e.name, e.tag)
}

func (c *Context) Header() http.Header {
	return c.req.Header
}

func (c *Context) CopyBody() ([]byte, error) {
	b, err := io.ReadAll(c.req.Body)
	if err != nil {
		return nil, err
	}
	c.req.Body = io.NopCloser(bytes.NewReader(b))
	return b, nil
}

func (c *Context) ParseQueryParam(object any) error {
	query := c.req.URL.Query()
	objectType := reflect.TypeOf(object)
	if objectType.Kind() != reflect.Pointer || objectType.Elem().Kind() != reflect.Struct {
		return errors.New("object should be pointer to struct")
	}
	if reflect.ValueOf(object).IsZero() {
		return errors.New("object cannot be nil pointer")
	}
	objectType = objectType.Elem()
	objectValue := reflect.Indirect(reflect.ValueOf(object))
	for i := 0; i < objectType.NumField(); i++ {
		structField := objectType.Field(i)
		tagName := structField.Tag.Get("query")
		valueList, ok := query[tagName]
		if !ok {
			continue
		}
		if len(valueList) == 0 {
			continue
		}
		lastValue := valueList[len(valueList)-1]
		structValue := objectValue.Field(i)
		if !structValue.CanSet() {
			return fmt.Errorf(
				"value is not setable, %w",
				NewSetFieldError(newFieldInfo(i, structField.Name, tagName, structField.Type.Kind())),
			)
		}
		switch structField.Type.Kind() {
		case reflect.String:
			structValue.SetString(lastValue)
		case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
			v, err := strconv.ParseInt(lastValue, 10, 64)
			if err != nil {
				return fmt.Errorf(
					"convert value failed, %w",
					NewParseFieldError(
						newFieldInfo(i, structField.Name, tagName, structField.Type.Kind()),
						lastValue,
					),
				)
			}
			structValue.SetInt(v)
		case reflect.Float32, reflect.Float64:
			v, err := strconv.ParseFloat(lastValue, 64)
			if err != nil {
				return fmt.Errorf(
					"convert value failed, %w",
					NewParseFieldError(
						newFieldInfo(i, structField.Name, tagName, structField.Type.Kind()),
						lastValue,
					),
				)
			}
			structValue.SetFloat(v)
		case reflect.Bool:
			value := strings.ToLower(lastValue)
			if value == "true" {
				structValue.SetBool(true)
			} else if value == "false" {
				structValue.SetBool(false)
			} else {
				return fmt.Errorf(
					"invalid bool value, %w",
					NewParseFieldError(
						newFieldInfo(i, structField.Name, tagName, structField.Type.Kind()),
						lastValue,
					),
				)
			}
		default:
			return errors.New("unsupported struct field type")
		}
	}
	return nil
}

func (c *Context) ParseJsonBody(v any) error {
	b, err := io.ReadAll(c.req.Body)
	if err != nil {
		// c.Logger.Error("read request body error", zap.Error(err))
		return err
	}
	err = json.Unmarshal(b, v)
	if err != nil {
		// c.Logger.Error("unmarshal request body error", zap.Error(err))
		return err
	}
	validator := validator.New()
	err = validator.Struct(v)
	if err != nil {
		return err
	}
	return nil
}

func (c *Context) ResponseJson(httpCode int, data any, header http.Header) error {
	c.resWriter.Header().Add("Content-Type", "application/json")
	for k, v := range header {
		c.resWriter.Header().Set(k, v[0])
	}
	c.resWriter.WriteHeader(httpCode)
	if data != nil {
		b, err := json.Marshal(data)
		if err != nil {
			c.Logger.Error("failed to marshal data", zap.Error(err))
			return err
		}
		_, err = c.resWriter.Write(b)
		if err != nil {
			c.Logger.Error("failed to write data", zap.Error(err))
			return err
		}
	}
	return nil
}

func (c *Context) AbortJson(httpCode int, data any, header http.Header) error {
	c.resWriter.Header().Add("Content-Type", "application/json")
	for k, v := range header {
		c.resWriter.Header().Set(k, v[0])
	}
	c.resWriter.WriteHeader(httpCode)
	if data != nil {
		b, err := json.Marshal(data)
		if err != nil {
			c.Logger.Error("failed to marshal data", zap.Error(err))
			return AbortError{err: err}
		}
		_, err = c.resWriter.Write(b)
		if err != nil {
			c.Logger.Error("failed to write data", zap.Error(err))
			return AbortError{err: err}
		}
	}
	return AbortError{}
}
