# \ResourceServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**resource_service_create_resource**](ResourceServiceApi.md#resource_service_create_resource) | **POST** /api/v1/resources | CreateResource creates a new resource.
[**resource_service_delete_resource**](ResourceServiceApi.md#resource_service_delete_resource) | **DELETE** /api/v1/{name_3} | DeleteResource deletes a resource by name.
[**resource_service_get_resource**](ResourceServiceApi.md#resource_service_get_resource) | **GET** /api/v1/{name_3} | GetResource returns a resource by name.
[**resource_service_get_resource_binary**](ResourceServiceApi.md#resource_service_get_resource_binary) | **GET** /file/{name}/{filename} | GetResourceBinary returns a resource binary by name.
[**resource_service_get_resource_by_uid**](ResourceServiceApi.md#resource_service_get_resource_by_uid) | **GET** /api/v1/resources:by-uid/{uid} | GetResourceByUid returns a resource by uid.
[**resource_service_list_resources**](ResourceServiceApi.md#resource_service_list_resources) | **GET** /api/v1/resources | ListResources lists all resources.
[**resource_service_update_resource**](ResourceServiceApi.md#resource_service_update_resource) | **PATCH** /api/v1/{resource_name} | UpdateResource updates a resource.



## resource_service_create_resource

> models::V1Resource resource_service_create_resource(resource)
CreateResource creates a new resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | [**V1Resource**](V1Resource.md) |  | [required] |

### Return type

[**models::V1Resource**](v1Resource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_service_delete_resource

> serde_json::Value resource_service_delete_resource(name_3)
DeleteResource deletes a resource by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_3** | **String** | The name of the resource. Format: resources/{id} id is the system generated unique identifier. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_service_get_resource

> models::V1Resource resource_service_get_resource(name_3)
GetResource returns a resource by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_3** | **String** | The name of the resource. Format: resources/{id} id is the system generated unique identifier. | [required] |

### Return type

[**models::V1Resource**](v1Resource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_service_get_resource_binary

> models::ApiHttpBody resource_service_get_resource_binary(name, filename, thumbnail)
GetResourceBinary returns a resource binary by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the resource. Format: resources/{id} id is the system generated unique identifier. | [required] |
**filename** | **String** | The filename of the resource. Mainly used for downloading. | [required] |
**thumbnail** | Option<**bool**> | A flag indicating if the thumbnail version of the resource should be returned |  |

### Return type

[**models::ApiHttpBody**](apiHttpBody.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_service_get_resource_by_uid

> models::V1Resource resource_service_get_resource_by_uid(uid)
GetResourceByUid returns a resource by uid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | The uid of the resource. | [required] |

### Return type

[**models::V1Resource**](v1Resource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_service_list_resources

> models::V1ListResourcesResponse resource_service_list_resources()
ListResources lists all resources.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::V1ListResourcesResponse**](v1ListResourcesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_service_update_resource

> models::V1Resource resource_service_update_resource(resource_name, resource)
UpdateResource updates a resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_name** | **String** | The name of the resource. Format: resources/{id} id is the system generated unique identifier. | [required] |
**resource** | [**ResourceServiceUpdateResourceRequest**](ResourceServiceUpdateResourceRequest.md) |  | [required] |

### Return type

[**models::V1Resource**](v1Resource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

