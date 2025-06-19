# Server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timeout_seconds** | Option<**f64**> | This is the timeout in seconds for the request. Defaults to 20 seconds.  @default 20 | [optional]
**url** | Option<**String**> | This is where the request will be sent. | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | These are the headers to include in the request.  Each key-value pair represents a header name and its value. | [optional]
**backoff_plan** | Option<[**models::BackoffPlan**](BackoffPlan.md)> | This is the backoff plan if the request fails. Defaults to undefined (the request will not be retried).  @default undefined (the request will not be retried) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


