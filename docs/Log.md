# Log

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time** | **String** | This is the timestamp at which the log was written. | 
**org_id** | **String** | This is the unique identifier for the org that this log belongs to. | 
**r#type** | [**models::LogType**](LogType.md) |  | 
**webhook_type** | Option<**String**> | This is the type of the webhook, given the log is from a webhook. | [optional]
**resource** | Option<[**models::LogResource**](LogResource.md)> |  | [optional]
**request_duration_seconds** | Option<**f64**> | 'This is how long the request took. | [optional]
**request_started_at** | Option<**String**> | This is the timestamp at which the request began. | [optional]
**request_finished_at** | Option<**String**> | This is the timestamp at which the request finished. | [optional]
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is the body of the request. | [optional]
**request_http_method** | Option<[**models::LogRequestHttpMethod**](LogRequestHttpMethod.md)> |  | [optional]
**request_url** | Option<**String**> | This is the request URL. | [optional]
**request_path** | Option<**String**> | This is the request path. | [optional]
**request_query** | Option<**String**> | This is the request query. | [optional]
**response_http_code** | Option<**f64**> | This the HTTP status code of the response. | [optional]
**request_ip_address** | Option<**String**> | This is the request IP address. | [optional]
**request_origin** | Option<**String**> | This is the origin of the request | [optional]
**response_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is the body of the response. | [optional]
**request_headers** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | These are the headers of the request. | [optional]
**error** | Option<[**models::Error**](Error.md)> |  | [optional]
**assistant_id** | Option<**String**> | This is the ID of the assistant. | [optional]
**phone_number_id** | Option<**String**> | This is the ID of the phone number. | [optional]
**customer_id** | Option<**String**> | This is the ID of the customer. | [optional]
**squad_id** | Option<**String**> | This is the ID of the squad. | [optional]
**call_id** | Option<**String**> | This is the ID of the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


