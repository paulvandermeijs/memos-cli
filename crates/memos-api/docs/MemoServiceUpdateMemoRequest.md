# MemoServiceUpdateMemoRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | The user defined id of the memo. | [optional]
**row_status** | Option<[**models::V1RowStatus**](v1RowStatus.md)> |  | [optional]
**creator** | Option<**String**> |  | [optional]
**create_time** | Option<**String**> |  | [optional]
**update_time** | Option<**String**> |  | [optional]
**display_time** | Option<**String**> |  | [optional]
**content** | Option<**String**> |  | [optional]
**nodes** | Option<[**Vec<models::V1Node>**](v1Node.md)> |  | [optional][readonly]
**visibility** | Option<[**models::V1Visibility**](v1Visibility.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**pinned** | Option<**bool**> |  | [optional]
**resources** | Option<[**Vec<models::V1Resource>**](v1Resource.md)> |  | [optional]
**relations** | Option<[**Vec<models::V1MemoRelation>**](v1MemoRelation.md)> |  | [optional]
**reactions** | Option<[**Vec<models::V1Reaction>**](v1Reaction.md)> |  | [optional][readonly]
**property** | Option<[**models::V1MemoProperty**](v1MemoProperty.md)> |  | [optional]
**parent** | Option<**String**> |  | [optional][readonly]
**snippet** | Option<**String**> | The snippet of the memo content. Plain text only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


