# VoiceCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | This is the voice that was used during the call.  This matches one of the following: - `call.assistant.voice`, - `call.assistantId->voice`, - `call.squad[n].assistant.voice`, - `call.squad[n].assistantId->voice`, - `call.squadId->[n].assistant.voice`, - `call.squadId->[n].assistantId->voice`. | 
**characters** | **f64** | This is the number of characters that were generated during the call. These should be total characters used in the call for single assistant calls, while squad calls will have multiple voice costs one for each assistant that was used. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


