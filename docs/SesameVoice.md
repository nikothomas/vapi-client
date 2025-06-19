# SesameVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_enabled** | Option<**bool**> | This is the flag to toggle voice caching for the assistant. | [optional][default to true]
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | **String** | This is the provider-specific ID that will be used. | 
**model** | **String** | This is the model that will be used. | 
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]
**fallback_plan** | Option<[**models::FallbackPlan**](FallbackPlan.md)> | This is the plan for voice provider fallbacks in the event that the primary voice provider fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


