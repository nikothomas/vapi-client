# GladiaTranscriber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<[**models::GladiaTranscriberModel**](GladiaTranscriberModel.md)> |  | [optional]
**language_behaviour** | Option<[**models::GladiaTranscriberLanguageBehaviour**](GladiaTranscriberLanguageBehaviour.md)> |  | [optional]
**language** | Option<[**models::GladiaTranscriberLanguage**](GladiaTranscriberLanguage.md)> |  | [optional]
**transcription_hint** | Option<**String**> | Provides a custom vocabulary to the model to improve accuracy of transcribing context specific words, technical terms, names, etc. If empty, this argument is ignored. ⚠️ Warning ⚠️: Please be aware that the transcription_hint field has a character limit of 600. If you provide a transcription_hint longer than 600 characters, it will be automatically truncated to meet this limit. | [optional]
**prosody** | Option<**bool**> | If prosody is true, you will get a transcription that can contain prosodies i.e. (laugh) (giggles) (malefic laugh) (toss) (music)… Default value is false. | [optional]
**audio_enhancer** | Option<**bool**> | If true, audio will be pre-processed to improve accuracy but latency will increase. Default value is false. | [optional]
**fallback_plan** | Option<[**models::FallbackTranscriberPlan**](FallbackTranscriberPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


