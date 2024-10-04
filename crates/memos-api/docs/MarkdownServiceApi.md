# \MarkdownServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**markdown_service_get_link_metadata**](MarkdownServiceApi.md#markdown_service_get_link_metadata) | **GET** /api/v1/markdown/link:metadata | GetLinkMetadata returns metadata for a given link.
[**markdown_service_parse_markdown**](MarkdownServiceApi.md#markdown_service_parse_markdown) | **POST** /api/v1/markdown:parse | ParseMarkdown parses the given markdown content and returns a list of nodes.
[**markdown_service_restore_markdown_nodes**](MarkdownServiceApi.md#markdown_service_restore_markdown_nodes) | **POST** /api/v1/markdown/node:restore | RestoreMarkdownNodes restores the given nodes to markdown content.
[**markdown_service_stringify_markdown_nodes**](MarkdownServiceApi.md#markdown_service_stringify_markdown_nodes) | **POST** /api/v1/markdown/node:stringify | StringifyMarkdownNodes stringify the given nodes to plain text content.



## markdown_service_get_link_metadata

> models::V1LinkMetadata markdown_service_get_link_metadata(link)
GetLinkMetadata returns metadata for a given link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link** | Option<**String**> |  |  |

### Return type

[**models::V1LinkMetadata**](v1LinkMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## markdown_service_parse_markdown

> models::V1ParseMarkdownResponse markdown_service_parse_markdown(body)
ParseMarkdown parses the given markdown content and returns a list of nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1ParseMarkdownRequest**](V1ParseMarkdownRequest.md) |  | [required] |

### Return type

[**models::V1ParseMarkdownResponse**](v1ParseMarkdownResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## markdown_service_restore_markdown_nodes

> models::V1RestoreMarkdownNodesResponse markdown_service_restore_markdown_nodes(body)
RestoreMarkdownNodes restores the given nodes to markdown content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1RestoreMarkdownNodesRequest**](V1RestoreMarkdownNodesRequest.md) |  | [required] |

### Return type

[**models::V1RestoreMarkdownNodesResponse**](v1RestoreMarkdownNodesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## markdown_service_stringify_markdown_nodes

> models::V1StringifyMarkdownNodesResponse markdown_service_stringify_markdown_nodes(body)
StringifyMarkdownNodes stringify the given nodes to plain text content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1StringifyMarkdownNodesRequest**](V1StringifyMarkdownNodesRequest.md) |  | [required] |

### Return type

[**models::V1StringifyMarkdownNodesResponse**](v1StringifyMarkdownNodesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

