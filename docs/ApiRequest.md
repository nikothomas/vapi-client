# ApiRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**method** | **String** |  | 
**url** | **String** | Api endpoint to send requests to. | 
**headers** | Option<[**models::JsonSchema**](JsonSchema.md)> | These are the custom headers to include in the Api Request sent.  Each key-value pair represents a header name and its value. | [optional]
**body** | Option<[**models::JsonSchema**](JsonSchema.md)> | This defined the JSON body of your Api Request. For example, if `body_schema` included \"my_field\": \"my_gather_statement.user_age\", then the json body sent to the server would have that particular value assign to it. Right now, only data from gather statements are supported. | [optional]
**mode** | **String** | This is the mode of the Api Request. We only support BLOCKING and BACKGROUND for now. | 
**hooks** | Option<[**Vec<models::Hook>**](Hook.md)> | This is a list of hooks for a task. Each hook is a list of tasks to run on a trigger (such as on start, on failure, etc). Only Say is supported for now. | [optional]
**output** | Option<[**models::JsonSchema**](JsonSchema.md)> | This is the schema for the outputs of the Api Request. | [optional]
**name** | **String** |  | 
**metadata** | Option<[**serde_json::Value**](.md)> | This is for metadata you want to store on the task. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


