# FallbackNeuphonicVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | **String** | This is the provider-specific ID that will be used. | 
**model** | Option<[**models::FallbackNeuphonicVoiceModel**](FallbackNeuphonicVoiceModel.md)> |  | [optional]
**language** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | This is the language (ISO 639-1) that is enforced for the model. | 
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


