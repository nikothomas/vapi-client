# TranscriberCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of cost, always 'transcriber' for this class. | 
**transcriber** | [**serde_json::Value**](.md) | This is the transcriber that was used during the call.  This matches one of the below: - `call.assistant.transcriber`, - `call.assistantId->transcriber`, - `call.squad[n].assistant.transcriber`, - `call.squad[n].assistantId->transcriber`, - `call.squadId->[n].assistant.transcriber`, - `call.squadId->[n].assistantId->transcriber`. | 
**minutes** | **f64** | This is the minutes of `transcriber` usage. This should match `call.endedAt` - `call.startedAt` for single assistant calls, while squad calls will have multiple transcriber costs one for each assistant that was used. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


