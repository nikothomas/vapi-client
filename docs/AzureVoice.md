# AzureVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_enabled** | Option<**bool**> | This is the flag to toggle voice caching for the assistant. | [optional][default to true]
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::AzureVoiceVoiceId**](AzureVoice_voiceId.md) |  | 
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**fallback_plan** | Option<[**models::FallbackPlan**](FallbackPlan.md)> | This is the plan for voice provider fallbacks in the event that the primary voice provider fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


