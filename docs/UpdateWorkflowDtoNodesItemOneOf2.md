# UpdateWorkflowDtoNodesItemOneOf2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | [**models::ApiRequestMethod**](ApiRequestMethod.md) |  | 
**url** | **String** | Api endpoint to send requests to. | 
**headers** | Option<[**models::JsonSchema**](JsonSchema.md)> |  | [optional]
**body** | Option<[**models::JsonSchema**](JsonSchema.md)> |  | [optional]
**mode** | [**models::ApiRequestMode**](ApiRequestMode.md) |  | 
**hooks** | Option<[**Vec<models::Hook>**](Hook.md)> | This is a list of hooks for a task. Each hook is a list of tasks to run on a trigger (such as on start, on failure, etc). Only Say is supported for now. | [optional]
**output** | Option<[**models::JsonSchema**](JsonSchema.md)> |  | [optional]
**name** | **String** |  | 
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is for metadata you want to store on the task. | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


