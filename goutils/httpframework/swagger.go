package httpframework

import (
	"strings"
)

func ParseRouterDoc(docStr string) *Info {
	result := &Info{
		Version: InfoVersion_1_0_11,
	}
	parts := strings.Split(docStr, "\n")
	for _, line := range parts {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		lineParts := strings.SplitN(line, " ", 2)
		switch lineParts[0] {
		case "@title":
			if len(lineParts) > 1 {
				result.Title = lineParts[1]
			}
		case "@description":
			if len(lineParts) > 1 {
				result.Description = lineParts[1]
			}
		case "@termsOfService":
			if len(lineParts) > 1 {
				result.TermsOfService = lineParts[1]
			}
		case "@contact.email":
			if len(lineParts) > 1 {
				if result.Contact == nil {
					result.Contact = &Info_Contact{}
				}
				result.Contact.Email = lineParts[1]
			}
		case "@license.name":
			if len(lineParts) > 1 {
				if result.License == nil {
					result.License = &Info_License{}
				}
				result.License.Name = lineParts[1]
			}
		case "@license.url":
			if len(lineParts) > 1 {
				if result.License == nil {
					result.License = &Info_License{}
				}
				result.License.URL = lineParts[1]
			}
		}
	}
	return result
}

type OpenApiVersion string

const (
	OpenApiVersion_3_0_3 OpenApiVersion = "3.0.3"
)

type Swagger struct {
	OpenApiVersion OpenApiVersion `json:"openapi"`
	Info           *Info          `json:"info"`
	Paths          Paths          `json:"paths"`
	Components     *Components    `json:"components"`
}

type InfoVersion string

const (
	InfoVersion_1_0_11 InfoVersion = "1.0.11"
)

type Info struct {
	Title          string        `json:"title"`
	Description    string        `json:"description"`
	TermsOfService string        `json:"termsOfService,omitempty"`
	Contact        *Info_Contact `json:"contact,omitempty"`
	License        *Info_License `json:"license,omitempty"`
	Version        InfoVersion   `json:"version"`
}

type Info_Contact struct {
	Email string `json:"email"`
}

type Info_License struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

type Paths map[string]*PathUrls

type HttpMethod string

const (
	HttpMethod_Get     HttpMethod = "get"
	HttpMethod_Post    HttpMethod = "post"
	HttpMethod_Put     HttpMethod = "put"
	HttpMethod_Delete  HttpMethod = "delete"
	HttpMethod_Options HttpMethod = "options"
	HttpMethod_Head    HttpMethod = "head"
	HttpMethod_Patch   HttpMethod = "patch"
	HttpMethod_Trace   HttpMethod = "trace"
)

type PathUrls map[HttpMethod]*PathUrlItem

type PathUrlItem struct {
	Description string                  `json:"description,omitempty"`
	Summary     string                  `json:"summary,omitempty"`
	RequestBody RequestBodyInfo         `json:"requestBody,omitempty"`
	Responses   map[string]ResponseItem `json:"responses,omitempty"`
}

type ContentType string

const (
	ContentType_ApplicationJson ContentType = "application/json"
)

type RequestBodyInfo struct {
	Description string `json:"description,omitempty"`
	Required    *bool  `json:"required,omitempty"`
	Content     map[ContentType]struct {
		Schema ContentSchemaItem `json:"schema"`
	} `json:"content,omitempty"`
}

type ContentSchemaItem struct {
	Ref string `json:"$ref"`
}

type ResponseItem struct {
	Description string `json:"description,omitempty"`
	Content     map[ContentType]struct {
		Schema ContentSchemaItem `json:"schema"`
	} `json:"content,omitempty"`
}

type Components struct {
	Schemas map[string]SchemaInfo `json:"schemas"`
	// RequestBodies map[string]RequestBodyInfo `json:"requestBodies"`
}

type SchemaInfo struct {
	Type       string           `json:"type"`
	Properties SchemaProperties `json:"properties"`
}

type SchemaProperties map[string]*SchemaPropertiesItem

type SchemaPropertiesItem struct {
	Type    string        `json:"type"`
	Format  string        `json:"format,omitempty"`
	Example *interface{}  `json:"example,omitempty"`
	Enum    []interface{} `json:"enum,omitempty"`
}
