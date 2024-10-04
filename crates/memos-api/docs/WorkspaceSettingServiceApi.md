# \WorkspaceSettingServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workspace_setting_service_get_workspace_setting**](WorkspaceSettingServiceApi.md#workspace_setting_service_get_workspace_setting) | **GET** /api/v1/workspace/{name} | GetWorkspaceSetting returns the setting by name.
[**workspace_setting_service_set_workspace_setting**](WorkspaceSettingServiceApi.md#workspace_setting_service_set_workspace_setting) | **PATCH** /api/v1/workspace/{setting_name} | SetWorkspaceSetting updates the setting.



## workspace_setting_service_get_workspace_setting

> models::Apiv1WorkspaceSetting workspace_setting_service_get_workspace_setting(name)
GetWorkspaceSetting returns the setting by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The resource name of the workspace setting. Format: settings/{setting} | [required] |

### Return type

[**models::Apiv1WorkspaceSetting**](apiv1WorkspaceSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_setting_service_set_workspace_setting

> models::Apiv1WorkspaceSetting workspace_setting_service_set_workspace_setting(setting_name, setting)
SetWorkspaceSetting updates the setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**setting_name** | **String** | name is the name of the setting. Format: settings/{setting} | [required] |
**setting** | [**SettingIsTheSettingToUpdate**](SettingIsTheSettingToUpdate.md) | setting is the setting to update. | [required] |

### Return type

[**models::Apiv1WorkspaceSetting**](apiv1WorkspaceSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

