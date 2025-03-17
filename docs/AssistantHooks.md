# AssistantHooks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**on** | **String** | This is the event that triggers this hook | 
**filters** | Option<[**Vec<models::AssistantHookFilter>**](AssistantHookFilter.md)> | This is the set of filters that must match for the hook to trigger | [optional]
**r#do** | [**Vec<serde_json::Value>**](serde_json::Value.md) | This is the set of actions to perform when the hook triggers | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


