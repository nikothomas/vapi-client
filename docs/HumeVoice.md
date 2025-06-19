# HumeVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_enabled** | Option<**bool**> | This is the flag to toggle voice caching for the assistant. | [optional][default to true]
**provider** | **String** | This is the voice provider that will be used. | 
**model** | Option<**String**> | This is the model that will be used. | [optional]
**voice_id** | **String** | The ID of the particular voice you want to use. | 
**is_custom_hume_voice** | Option<**bool**> | Indicates whether the chosen voice is a preset Hume AI voice or a custom voice. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]
**description** | Option<**String**> | Natural language instructions describing how the synthesized speech should sound, including but not limited to tone, intonation, pacing, and accent (e.g., 'a soft, gentle voice with a strong British accent').  If a Voice is specified in the request, this description serves as acting instructions. If no Voice is specified, a new voice is generated based on this description. | [optional]
**fallback_plan** | Option<[**models::FallbackPlan**](FallbackPlan.md)> | This is the plan for voice provider fallbacks in the event that the primary voice provider fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


