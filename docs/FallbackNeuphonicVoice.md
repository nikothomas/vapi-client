# FallbackNeuphonicVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_enabled** | Option<**bool**> | This is the flag to toggle voice caching for the assistant. | [optional][default to true]
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::NeuphonicVoiceVoiceId**](NeuphonicVoice_voiceId.md) |  | 
**model** | Option<**String**> | This is the model that will be used. Defaults to 'neu_fast' if not specified. | [optional]
**language** | [**serde_json::Value**](.md) | This is the language (ISO 639-1) that is enforced for the model. | 
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


