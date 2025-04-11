# ModelCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | This is the model that was used during the call.  This matches one of the following: - `call.assistant.model`, - `call.assistantId->model`, - `call.squad[n].assistant.model`, - `call.squad[n].assistantId->model`, - `call.squadId->[n].assistant.model`, - `call.squadId->[n].assistantId->model`. | 
**prompt_tokens** | **f64** | This is the number of prompt tokens used in the call. These should be total prompt tokens used in the call for single assistant calls, while squad calls will have multiple model costs one for each assistant that was used. | 
**completion_tokens** | **f64** | This is the number of completion tokens generated in the call. These should be total completion tokens used in the call for single assistant calls, while squad calls will have multiple model costs one for each assistant that was used. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


