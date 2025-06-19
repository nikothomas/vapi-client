# FallbackSmallestAiVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_enabled** | Option<**bool**> | This is the flag to toggle voice caching for the assistant. | [optional][default to true]
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::SmallestAiVoiceVoiceId**](SmallestAIVoice_voiceId.md) |  | 
**model** | Option<**String**> | Smallest AI voice model to use. Defaults to 'lightning' when not specified. | [optional]
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


