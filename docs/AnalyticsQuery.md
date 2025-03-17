# AnalyticsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table** | **String** | This is the table you want to query. | 
**group_by** | Option<**Vec<String>**> | This is the list of columns you want to group by. | [optional]
**name** | **String** | This is the name of the query. This will be used to identify the query in the response. | 
**time_range** | Option<[**models::TimeRange**](TimeRange.md)> | This is the time range for the query. | [optional]
**operations** | [**Vec<models::AnalyticsOperation>**](AnalyticsOperation.md) | This is the list of operations you want to perform. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


