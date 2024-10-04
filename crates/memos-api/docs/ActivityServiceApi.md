# \ActivityServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activity_service_get_activity**](ActivityServiceApi.md#activity_service_get_activity) | **GET** /api/v1/{name} | GetActivity returns the activity with the given id.



## activity_service_get_activity

> models::V1Activity activity_service_get_activity(name)
GetActivity returns the activity with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the activity. Format: activities/{id} | [required] |

### Return type

[**models::V1Activity**](v1Activity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

