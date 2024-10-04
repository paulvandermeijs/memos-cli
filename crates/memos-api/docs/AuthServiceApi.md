# \AuthServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_service_get_auth_status**](AuthServiceApi.md#auth_service_get_auth_status) | **POST** /api/v1/auth/status | GetAuthStatus returns the current auth status of the user.
[**auth_service_sign_in**](AuthServiceApi.md#auth_service_sign_in) | **POST** /api/v1/auth/signin | SignIn signs in the user with the given username and password.
[**auth_service_sign_in_with_sso**](AuthServiceApi.md#auth_service_sign_in_with_sso) | **POST** /api/v1/auth/signin/sso | SignInWithSSO signs in the user with the given SSO code.
[**auth_service_sign_out**](AuthServiceApi.md#auth_service_sign_out) | **POST** /api/v1/auth/signout | SignOut signs out the user.
[**auth_service_sign_up**](AuthServiceApi.md#auth_service_sign_up) | **POST** /api/v1/auth/signup | SignUp signs up the user with the given username and password.



## auth_service_get_auth_status

> models::V1User auth_service_get_auth_status()
GetAuthStatus returns the current auth status of the user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::V1User**](v1User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_service_sign_in

> models::V1User auth_service_sign_in(username, password, never_expire)
SignIn signs in the user with the given username and password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> | The username to sign in with. |  |
**password** | Option<**String**> | The password to sign in with. |  |
**never_expire** | Option<**bool**> | Whether the session should never expire. |  |

### Return type

[**models::V1User**](v1User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_service_sign_in_with_sso

> models::V1User auth_service_sign_in_with_sso(idp_id, code, redirect_uri)
SignInWithSSO signs in the user with the given SSO code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idp_id** | Option<**i32**> | The ID of the SSO provider. |  |
**code** | Option<**String**> | The code to sign in with. |  |
**redirect_uri** | Option<**String**> | The redirect URI. |  |

### Return type

[**models::V1User**](v1User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_service_sign_out

> serde_json::Value auth_service_sign_out()
SignOut signs out the user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_service_sign_up

> models::V1User auth_service_sign_up(username, password)
SignUp signs up the user with the given username and password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> | The username to sign up with. |  |
**password** | Option<**String**> | The password to sign up with. |  |

### Return type

[**models::V1User**](v1User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

