# \MemoServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**memo_service_create_memo**](MemoServiceApi.md#memo_service_create_memo) | **POST** /api/v1/memos | CreateMemo creates a memo.
[**memo_service_create_memo_comment**](MemoServiceApi.md#memo_service_create_memo_comment) | **POST** /api/v1/{name}/comments | CreateMemoComment creates a comment for a memo.
[**memo_service_delete_memo**](MemoServiceApi.md#memo_service_delete_memo) | **DELETE** /api/v1/{name_4} | DeleteMemo deletes a memo.
[**memo_service_delete_memo_reaction**](MemoServiceApi.md#memo_service_delete_memo_reaction) | **DELETE** /api/v1/reactions/{reactionId} | DeleteMemoReaction deletes a reaction for a memo.
[**memo_service_delete_memo_tag**](MemoServiceApi.md#memo_service_delete_memo_tag) | **DELETE** /api/v1/{parent}/tags/{tag} | DeleteMemoTag deletes a tag for a memo.
[**memo_service_get_memo**](MemoServiceApi.md#memo_service_get_memo) | **GET** /api/v1/{name_4} | GetMemo gets a memo.
[**memo_service_get_memo_by_uid**](MemoServiceApi.md#memo_service_get_memo_by_uid) | **GET** /api/v1/memos:by-uid/{uid} | GetMemoByUid gets a memo by uid
[**memo_service_list_memo_comments**](MemoServiceApi.md#memo_service_list_memo_comments) | **GET** /api/v1/{name}/comments | ListMemoComments lists comments for a memo.
[**memo_service_list_memo_properties**](MemoServiceApi.md#memo_service_list_memo_properties) | **GET** /api/v1/{name}/properties | ListMemoProperties lists memo properties.
[**memo_service_list_memo_reactions**](MemoServiceApi.md#memo_service_list_memo_reactions) | **GET** /api/v1/{name}/reactions | ListMemoReactions lists reactions for a memo.
[**memo_service_list_memo_relations**](MemoServiceApi.md#memo_service_list_memo_relations) | **GET** /api/v1/{name}/relations | ListMemoRelations lists relations for a memo.
[**memo_service_list_memo_resources**](MemoServiceApi.md#memo_service_list_memo_resources) | **GET** /api/v1/{name}/resources | ListMemoResources lists resources for a memo.
[**memo_service_list_memo_tags**](MemoServiceApi.md#memo_service_list_memo_tags) | **GET** /api/v1/{parent}/tags | ListMemoTags lists tags for a memo.
[**memo_service_list_memos**](MemoServiceApi.md#memo_service_list_memos) | **GET** /api/v1/memos | ListMemos lists memos with pagination and filter.
[**memo_service_rebuild_memo_property**](MemoServiceApi.md#memo_service_rebuild_memo_property) | **POST** /api/v1/{name}/properties:rebuild | RebuildMemoProperty rebuilds a memo property.
[**memo_service_rename_memo_tag**](MemoServiceApi.md#memo_service_rename_memo_tag) | **PATCH** /api/v1/{parent}/tags:rename | RenameMemoTag renames a tag for a memo.
[**memo_service_set_memo_relations**](MemoServiceApi.md#memo_service_set_memo_relations) | **PATCH** /api/v1/{name}/relations | SetMemoRelations sets relations for a memo.
[**memo_service_set_memo_resources**](MemoServiceApi.md#memo_service_set_memo_resources) | **PATCH** /api/v1/{name}/resources | SetMemoResources sets resources for a memo.
[**memo_service_update_memo**](MemoServiceApi.md#memo_service_update_memo) | **PATCH** /api/v1/{memo_name} | UpdateMemo updates a memo.
[**memo_service_upsert_memo_reaction**](MemoServiceApi.md#memo_service_upsert_memo_reaction) | **POST** /api/v1/{name}/reactions | UpsertMemoReaction upserts a reaction for a memo.



## memo_service_create_memo

> models::V1Memo memo_service_create_memo(body)
CreateMemo creates a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1CreateMemoRequest**](V1CreateMemoRequest.md) |  | [required] |

### Return type

[**models::V1Memo**](v1Memo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_create_memo_comment

> models::V1Memo memo_service_create_memo_comment(name, comment)
CreateMemoComment creates a comment for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |
**comment** | [**V1CreateMemoRequest**](V1CreateMemoRequest.md) |  | [required] |

### Return type

[**models::V1Memo**](v1Memo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_delete_memo

> serde_json::Value memo_service_delete_memo(name_4)
DeleteMemo deletes a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_4** | **String** | The name of the memo. Format: memos/{id} | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_delete_memo_reaction

> serde_json::Value memo_service_delete_memo_reaction(reaction_id)
DeleteMemoReaction deletes a reaction for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reaction_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_delete_memo_tag

> serde_json::Value memo_service_delete_memo_tag(parent, tag, delete_related_memos)
DeleteMemoTag deletes a tag for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent** | **String** | The parent, who owns the tags. Format: memos/{id}. Use \"memos/-\" to delete all tags. | [required] |
**tag** | **String** |  | [required] |
**delete_related_memos** | Option<**bool**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_get_memo

> models::V1Memo memo_service_get_memo(name_4)
GetMemo gets a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_4** | **String** | The name of the memo. Format: memos/{id} | [required] |

### Return type

[**models::V1Memo**](v1Memo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_get_memo_by_uid

> models::V1Memo memo_service_get_memo_by_uid(uid)
GetMemoByUid gets a memo by uid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | The uid of the memo. | [required] |

### Return type

[**models::V1Memo**](v1Memo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_list_memo_comments

> models::V1ListMemoCommentsResponse memo_service_list_memo_comments(name)
ListMemoComments lists comments for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |

### Return type

[**models::V1ListMemoCommentsResponse**](v1ListMemoCommentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_list_memo_properties

> models::V1ListMemoPropertiesResponse memo_service_list_memo_properties(name)
ListMemoProperties lists memo properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id}. Use \"memos/-\" to list all properties. | [required] |

### Return type

[**models::V1ListMemoPropertiesResponse**](v1ListMemoPropertiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_list_memo_reactions

> models::V1ListMemoReactionsResponse memo_service_list_memo_reactions(name)
ListMemoReactions lists reactions for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |

### Return type

[**models::V1ListMemoReactionsResponse**](v1ListMemoReactionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_list_memo_relations

> models::V1ListMemoRelationsResponse memo_service_list_memo_relations(name)
ListMemoRelations lists relations for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |

### Return type

[**models::V1ListMemoRelationsResponse**](v1ListMemoRelationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_list_memo_resources

> models::V1ListMemoResourcesResponse memo_service_list_memo_resources(name)
ListMemoResources lists resources for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |

### Return type

[**models::V1ListMemoResourcesResponse**](v1ListMemoResourcesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_list_memo_tags

> models::V1ListMemoTagsResponse memo_service_list_memo_tags(parent, filter)
ListMemoTags lists tags for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent** | **String** | The parent, who owns the tags. Format: memos/{id}. Use \"memos/-\" to list all tags. | [required] |
**filter** | Option<**String**> | Filter is used to filter memos. Format: \"creator == 'users/{uid}' && visibilities == ['PUBLIC', 'PROTECTED']\" |  |

### Return type

[**models::V1ListMemoTagsResponse**](v1ListMemoTagsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_list_memos

> models::V1ListMemosResponse memo_service_list_memos(page_size, page_token, filter)
ListMemos lists memos with pagination and filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The maximum number of memos to return. |  |
**page_token** | Option<**String**> | A page token, received from a previous `ListMemos` call. Provide this to retrieve the subsequent page. |  |
**filter** | Option<**String**> | Filter is used to filter memos returned in the list. Format: \"creator == 'users/{uid}' && visibilities == ['PUBLIC', 'PROTECTED']\" |  |

### Return type

[**models::V1ListMemosResponse**](v1ListMemosResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_rebuild_memo_property

> serde_json::Value memo_service_rebuild_memo_property(name, body)
RebuildMemoProperty rebuilds a memo property.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id}. Use \"memos/-\" to rebuild all memos. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_rename_memo_tag

> serde_json::Value memo_service_rename_memo_tag(parent, body)
RenameMemoTag renames a tag for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent** | **String** | The parent, who owns the tags. Format: memos/{id}. Use \"memos/-\" to rename all tags. | [required] |
**body** | [**MemoServiceRenameMemoTagBody**](MemoServiceRenameMemoTagBody.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_set_memo_relations

> serde_json::Value memo_service_set_memo_relations(name, body)
SetMemoRelations sets relations for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |
**body** | [**MemoServiceSetMemoRelationsBody**](MemoServiceSetMemoRelationsBody.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_set_memo_resources

> serde_json::Value memo_service_set_memo_resources(name, body)
SetMemoResources sets resources for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |
**body** | [**MemoServiceSetMemoResourcesBody**](MemoServiceSetMemoResourcesBody.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_update_memo

> models::V1Memo memo_service_update_memo(memo_name, memo)
UpdateMemo updates a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**memo_name** | **String** | The name of the memo. Format: memos/{id} id is the system generated id. | [required] |
**memo** | [**MemoServiceUpdateMemoRequest**](MemoServiceUpdateMemoRequest.md) |  | [required] |

### Return type

[**models::V1Memo**](v1Memo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memo_service_upsert_memo_reaction

> models::V1Reaction memo_service_upsert_memo_reaction(name, body)
UpsertMemoReaction upserts a reaction for a memo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the memo. Format: memos/{id} | [required] |
**body** | [**MemoServiceUpsertMemoReactionBody**](MemoServiceUpsertMemoReactionBody.md) |  | [required] |

### Return type

[**models::V1Reaction**](v1Reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

