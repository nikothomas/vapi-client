# NeuphonicVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_id** | **String** | This is the provider-specific ID that will be used. | 
**model** | Option<[**models::NeuphonicVoiceModel**](NeuphonicVoiceModel.md)> |  | [optional]
**language** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | This is the language (ISO 639-1) that is enforced for the model. | 
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> |  | [optional]
**fallback_plan** | Option<[**models::FallbackPlan**](FallbackPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


