# AnalyticsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table** | [**models::AnalyticsQueryTable**](AnalyticsQueryTable.md) |  | 
**group_by** | Option<[**Vec<models::AnalyticsQueryGroupByItem>**](AnalyticsQueryGroupByItem.md)> | This is the list of columns you want to group by. | [optional]
**name** | **String** | This is the name of the query. This will be used to identify the query in the response. | 
**time_range** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**operations** | [**Vec<models::AnalyticsOperation>**](AnalyticsOperation.md) | This is the list of operations you want to perform. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


