# FallbackGladiaTranscriber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the transcription provider that will be used. | 
**model** | Option<**String**> |  | [optional]
**language_behaviour** | Option<**String**> |  | [optional]
**language** | Option<**String**> | Defines the language to use for the transcription. Required when languageBehaviour is 'manual'. | [optional]
**languages** | Option<**String**> | Defines the languages to use for the transcription. Required when languageBehaviour is 'manual'. | [optional]
**transcription_hint** | Option<**String**> | Provides a custom vocabulary to the model to improve accuracy of transcribing context specific words, technical terms, names, etc. If empty, this argument is ignored. ⚠️ Warning ⚠️: Please be aware that the transcription_hint field has a character limit of 600. If you provide a transcription_hint longer than 600 characters, it will be automatically truncated to meet this limit. | [optional]
**prosody** | Option<**bool**> | If prosody is true, you will get a transcription that can contain prosodies i.e. (laugh) (giggles) (malefic laugh) (toss) (music)… Default value is false. | [optional]
**audio_enhancer** | Option<**bool**> | If true, audio will be pre-processed to improve accuracy but latency will increase. Default value is false. | [optional]
**confidence_threshold** | Option<**f64**> | Transcripts below this confidence threshold will be discarded.  @default 0.4 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


