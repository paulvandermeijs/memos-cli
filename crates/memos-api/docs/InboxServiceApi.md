# \InboxServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inbox_service_delete_inbox**](InboxServiceApi.md#inbox_service_delete_inbox) | **DELETE** /api/v1/{name_2} | DeleteInbox deletes an inbox.
[**inbox_service_list_inboxes**](InboxServiceApi.md#inbox_service_list_inboxes) | **GET** /api/v1/inboxes | ListInboxes lists inboxes for a user.
[**inbox_service_update_inbox**](InboxServiceApi.md#inbox_service_update_inbox) | **PATCH** /api/v1/{inbox_name} | UpdateInbox updates an inbox.



## inbox_service_delete_inbox

> serde_json::Value inbox_service_delete_inbox(name_2)
DeleteInbox deletes an inbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_2** | **String** | The name of the inbox to delete. Format: inboxes/{id} | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inbox_service_list_inboxes

> models::V1ListInboxesResponse inbox_service_list_inboxes(user)
ListInboxes lists inboxes for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | Option<**String**> | Format: users/{id} |  |

### Return type

[**models::V1ListInboxesResponse**](v1ListInboxesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inbox_service_update_inbox

> models::V1Inbox inbox_service_update_inbox(inbox_name, inbox)
UpdateInbox updates an inbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_name** | **String** | The name of the inbox. Format: inboxes/{id} | [required] |
**inbox** | [**InboxServiceUpdateInboxRequest**](InboxServiceUpdateInboxRequest.md) |  | [required] |

### Return type

[**models::V1Inbox**](v1Inbox.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

