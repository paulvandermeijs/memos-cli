# \UserServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_service_create_user**](UserServiceApi.md#user_service_create_user) | **POST** /api/v1/users | CreateUser creates a new user.
[**user_service_create_user_access_token**](UserServiceApi.md#user_service_create_user_access_token) | **POST** /api/v1/{name}/access_tokens | CreateUserAccessToken creates a new access token for a user.
[**user_service_delete_user**](UserServiceApi.md#user_service_delete_user) | **DELETE** /api/v1/{name} | DeleteUser deletes a user.
[**user_service_delete_user_access_token**](UserServiceApi.md#user_service_delete_user_access_token) | **DELETE** /api/v1/{name}/access_tokens/{accessToken} | DeleteUserAccessToken deletes an access token for a user.
[**user_service_get_user**](UserServiceApi.md#user_service_get_user) | **GET** /api/v1/{name_1} | GetUser gets a user by name.
[**user_service_get_user_avatar_binary**](UserServiceApi.md#user_service_get_user_avatar_binary) | **GET** /file/{name}/avatar | GetUserAvatarBinary gets the avatar of a user.
[**user_service_get_user_setting**](UserServiceApi.md#user_service_get_user_setting) | **GET** /api/v1/{name}/setting | GetUserSetting gets the setting of a user.
[**user_service_list_user_access_tokens**](UserServiceApi.md#user_service_list_user_access_tokens) | **GET** /api/v1/{name}/access_tokens | ListUserAccessTokens returns a list of access tokens for a user.
[**user_service_list_users**](UserServiceApi.md#user_service_list_users) | **GET** /api/v1/users | ListUsers returns a list of users.
[**user_service_search_users**](UserServiceApi.md#user_service_search_users) | **GET** /api/v1/users:search | SearchUsers searches users by filter.
[**user_service_update_user**](UserServiceApi.md#user_service_update_user) | **PATCH** /api/v1/{user_name} | UpdateUser updates a user.
[**user_service_update_user_setting**](UserServiceApi.md#user_service_update_user_setting) | **PATCH** /api/v1/{setting_name} | UpdateUserSetting updates the setting of a user.



## user_service_create_user

> models::V1User user_service_create_user(user)
CreateUser creates a new user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | [**V1User**](V1User.md) |  | [required] |

### Return type

[**models::V1User**](v1User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_create_user_access_token

> models::V1UserAccessToken user_service_create_user_access_token(name, body)
CreateUserAccessToken creates a new access token for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the user. Format: users/{id} | [required] |
**body** | [**UserServiceCreateUserAccessTokenBody**](UserServiceCreateUserAccessTokenBody.md) |  | [required] |

### Return type

[**models::V1UserAccessToken**](v1UserAccessToken.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_delete_user

> serde_json::Value user_service_delete_user(name)
DeleteUser deletes a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the user. Format: users/{id} | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_delete_user_access_token

> serde_json::Value user_service_delete_user_access_token(name, access_token)
DeleteUserAccessToken deletes an access token for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the user. Format: users/{id} | [required] |
**access_token** | **String** | access_token is the access token to delete. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_get_user

> models::V1User user_service_get_user(name_1)
GetUser gets a user by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_1** | **String** | The name of the user. Format: users/{id} | [required] |

### Return type

[**models::V1User**](v1User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_get_user_avatar_binary

> models::ApiHttpBody user_service_get_user_avatar_binary(name, http_body_period_content_type, http_body_period_data)
GetUserAvatarBinary gets the avatar of a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the user. Format: users/{id} | [required] |
**http_body_period_content_type** | Option<**String**> | The HTTP Content-Type header value specifying the content type of the body. |  |
**http_body_period_data** | Option<**String**> | The HTTP request/response body as raw binary. |  |

### Return type

[**models::ApiHttpBody**](apiHttpBody.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_get_user_setting

> models::Apiv1UserSetting user_service_get_user_setting(name)
GetUserSetting gets the setting of a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the user. Format: users/{id} | [required] |

### Return type

[**models::Apiv1UserSetting**](apiv1UserSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_list_user_access_tokens

> models::V1ListUserAccessTokensResponse user_service_list_user_access_tokens(name)
ListUserAccessTokens returns a list of access tokens for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the user. Format: users/{id} | [required] |

### Return type

[**models::V1ListUserAccessTokensResponse**](v1ListUserAccessTokensResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_list_users

> models::V1ListUsersResponse user_service_list_users()
ListUsers returns a list of users.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::V1ListUsersResponse**](v1ListUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_search_users

> models::V1SearchUsersResponse user_service_search_users(filter)
SearchUsers searches users by filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter is used to filter users returned in the list. Format: \"username == 'frank'\" |  |

### Return type

[**models::V1SearchUsersResponse**](v1SearchUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_update_user

> models::V1User user_service_update_user(user_name, user)
UpdateUser updates a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | The name of the user. Format: users/{id} | [required] |
**user** | [**UserServiceUpdateUserRequest**](UserServiceUpdateUserRequest.md) |  | [required] |

### Return type

[**models::V1User**](v1User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_service_update_user_setting

> models::Apiv1UserSetting user_service_update_user_setting(setting_name, setting)
UpdateUserSetting updates the setting of a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**setting_name** | **String** | The name of the user. Format: users/{id} | [required] |
**setting** | [**UserServiceUpdateUserSettingRequest**](UserServiceUpdateUserSettingRequest.md) |  | [required] |

### Return type

[**models::Apiv1UserSetting**](apiv1UserSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

