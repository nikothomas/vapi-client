# FallbackOpenAiVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::OpenAiVoiceVoiceId**](OpenAIVoice_voiceId.md) |  | 
**model** | Option<**String**> | This is the model that will be used for text-to-speech. | [optional]
**instructions** | Option<**String**> | This is a prompt that allows you to control the voice of your generated audio. Does not work with 'tts-1' or 'tts-1-hd' models. | [optional]
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


