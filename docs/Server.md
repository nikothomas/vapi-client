# Server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timeout_seconds** | Option<**f64**> | This is the timeout in seconds for the request to your server. Defaults to 20 seconds.  @default 20 | [optional]
**url** | **String** | API endpoint to send requests to. | 
**secret** | Option<**String**> | This is the secret you can set that Vapi will send with every request to your server. Will be sent as a header called x-vapi-secret.  Same precedence logic as server. | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | These are the custom headers to include in the request sent to your server.  Each key-value pair represents a header name and its value. | [optional]
**backoff_plan** | Option<[**models::BackoffPlan**](BackoffPlan.md)> | This is the backoff plan to use if the request fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


