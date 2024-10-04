# \WebhookServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhook_service_create_webhook**](WebhookServiceApi.md#webhook_service_create_webhook) | **POST** /api/v1/webhooks | CreateWebhook creates a new webhook.
[**webhook_service_delete_webhook**](WebhookServiceApi.md#webhook_service_delete_webhook) | **DELETE** /api/v1/webhooks/{id} | DeleteWebhook deletes a webhook by id.
[**webhook_service_get_webhook**](WebhookServiceApi.md#webhook_service_get_webhook) | **GET** /api/v1/webhooks/{id} | GetWebhook returns a webhook by id.
[**webhook_service_list_webhooks**](WebhookServiceApi.md#webhook_service_list_webhooks) | **GET** /api/v1/webhooks | ListWebhooks returns a list of webhooks.
[**webhook_service_update_webhook**](WebhookServiceApi.md#webhook_service_update_webhook) | **PATCH** /api/v1/webhooks/{webhook_id} | UpdateWebhook updates a webhook.



## webhook_service_create_webhook

> models::V1Webhook webhook_service_create_webhook(body)
CreateWebhook creates a new webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1CreateWebhookRequest**](V1CreateWebhookRequest.md) |  | [required] |

### Return type

[**models::V1Webhook**](v1Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_service_delete_webhook

> serde_json::Value webhook_service_delete_webhook(id)
DeleteWebhook deletes a webhook by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_service_get_webhook

> models::V1Webhook webhook_service_get_webhook(id)
GetWebhook returns a webhook by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::V1Webhook**](v1Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_service_list_webhooks

> models::V1ListWebhooksResponse webhook_service_list_webhooks(creator_id)
ListWebhooks returns a list of webhooks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**creator_id** | Option<**i32**> |  |  |

### Return type

[**models::V1ListWebhooksResponse**](v1ListWebhooksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_service_update_webhook

> models::V1Webhook webhook_service_update_webhook(webhook_id, webhook)
UpdateWebhook updates a webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **i32** |  | [required] |
**webhook** | [**WebhookServiceUpdateWebhookRequest**](WebhookServiceUpdateWebhookRequest.md) |  | [required] |

### Return type

[**models::V1Webhook**](v1Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

