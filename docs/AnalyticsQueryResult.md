# AnalyticsQueryResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | This is the unique key for the query. | 
**time_range** | [**models::TimeRange**](TimeRange.md) | This is the time range for the query. | 
**result** | [**Vec<serde_json::Value>**](serde_json::Value.md) | This is the result of the query, a list of unique groups with result of their aggregations.  Example: \"result\": [   { \"date\": \"2023-01-01\", \"assistantId\": \"123\", \"endedReason\": \"customer-ended-call\", \"sumDuration\": 120, \"avgCost\": 10.5 },   { \"date\": \"2023-01-02\", \"assistantId\": \"123\", \"endedReason\": \"customer-did-not-give-microphone-permission\", \"sumDuration\": 0, \"avgCost\": 0 },   // Additional results ] | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


