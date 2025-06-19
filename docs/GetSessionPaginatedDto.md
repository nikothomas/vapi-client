# GetSessionPaginatedDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | This is the name of the session to filter by. | [optional]
**assistant_id** | Option<**String**> | This is the ID of the assistant to filter sessions by. | [optional]
**workflow_id** | Option<**String**> | This is the ID of the workflow to filter sessions by. | [optional]
**page** | Option<**f64**> | This is the page number to return. Defaults to 1. | [optional]
**sort_order** | Option<**String**> | This is the sort order for pagination. Defaults to 'DESC'. | [optional]
**limit** | Option<**f64**> | This is the maximum number of items to return. Defaults to 100. | [optional]
**created_at_gt** | Option<**String**> | This will return items where the createdAt is greater than the specified value. | [optional]
**created_at_lt** | Option<**String**> | This will return items where the createdAt is less than the specified value. | [optional]
**created_at_ge** | Option<**String**> | This will return items where the createdAt is greater than or equal to the specified value. | [optional]
**created_at_le** | Option<**String**> | This will return items where the createdAt is less than or equal to the specified value. | [optional]
**updated_at_gt** | Option<**String**> | This will return items where the updatedAt is greater than the specified value. | [optional]
**updated_at_lt** | Option<**String**> | This will return items where the updatedAt is less than the specified value. | [optional]
**updated_at_ge** | Option<**String**> | This will return items where the updatedAt is greater than or equal to the specified value. | [optional]
**updated_at_le** | Option<**String**> | This will return items where the updatedAt is less than or equal to the specified value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


