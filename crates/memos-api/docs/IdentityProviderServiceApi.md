# \IdentityProviderServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**identity_provider_service_create_identity_provider**](IdentityProviderServiceApi.md#identity_provider_service_create_identity_provider) | **POST** /api/v1/identityProviders | CreateIdentityProvider creates an identity provider.
[**identity_provider_service_delete_identity_provider**](IdentityProviderServiceApi.md#identity_provider_service_delete_identity_provider) | **DELETE** /api/v1/{name_1} | DeleteIdentityProvider deletes an identity provider.
[**identity_provider_service_get_identity_provider**](IdentityProviderServiceApi.md#identity_provider_service_get_identity_provider) | **GET** /api/v1/{name_2} | GetIdentityProvider gets an identity provider.
[**identity_provider_service_list_identity_providers**](IdentityProviderServiceApi.md#identity_provider_service_list_identity_providers) | **GET** /api/v1/identityProviders | ListIdentityProviders lists identity providers.
[**identity_provider_service_update_identity_provider**](IdentityProviderServiceApi.md#identity_provider_service_update_identity_provider) | **PATCH** /api/v1/{identityProvider_name} | UpdateIdentityProvider updates an identity provider.



## identity_provider_service_create_identity_provider

> models::Apiv1IdentityProvider identity_provider_service_create_identity_provider(identity_provider)
CreateIdentityProvider creates an identity provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider** | [**Apiv1IdentityProvider**](Apiv1IdentityProvider.md) | The identityProvider to create. | [required] |

### Return type

[**models::Apiv1IdentityProvider**](apiv1IdentityProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_provider_service_delete_identity_provider

> serde_json::Value identity_provider_service_delete_identity_provider(name_1)
DeleteIdentityProvider deletes an identity provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_1** | **String** | The name of the identityProvider to delete. Format: identityProviders/{id} | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_provider_service_get_identity_provider

> models::Apiv1IdentityProvider identity_provider_service_get_identity_provider(name_2)
GetIdentityProvider gets an identity provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_2** | **String** | The name of the identityProvider to get. Format: identityProviders/{id} | [required] |

### Return type

[**models::Apiv1IdentityProvider**](apiv1IdentityProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_provider_service_list_identity_providers

> models::V1ListIdentityProvidersResponse identity_provider_service_list_identity_providers()
ListIdentityProviders lists identity providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::V1ListIdentityProvidersResponse**](v1ListIdentityProvidersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_provider_service_update_identity_provider

> models::Apiv1IdentityProvider identity_provider_service_update_identity_provider(identity_provider_name, identity_provider)
UpdateIdentityProvider updates an identity provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_name** | **String** | The name of the identityProvider. Format: identityProviders/{id} | [required] |
**identity_provider** | [**TheIdentityProviderToUpdate**](TheIdentityProviderToUpdate.md) | The identityProvider to update. | [required] |

### Return type

[**models::Apiv1IdentityProvider**](apiv1IdentityProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

