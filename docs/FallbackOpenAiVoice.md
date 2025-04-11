# FallbackOpenAiVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_id** | [**models::FallbackOpenAiVoiceId**](FallbackOpenAIVoiceId.md) |  | 
**model** | Option<[**models::FallbackOpenAiVoiceModel**](FallbackOpenAiVoiceModel.md)> |  | [optional]
**instructions** | Option<**String**> | This is a prompt that allows you to control the voice of your generated audio. Does not work with 'tts-1' or 'tts-1-hd' models. | [optional]
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


