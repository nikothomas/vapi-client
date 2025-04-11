# FallbackAssemblyAiTranscriber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**language** | Option<**String**> | This is the language that will be set for the transcription. | [optional]
**realtime_url** | Option<**String**> | The WebSocket URL that the transcriber connects to. | [optional]
**word_boost** | Option<**Vec<String>**> | Add up to 2500 characters of custom vocabulary. | [optional]
**end_utterance_silence_threshold** | Option<**f64**> | The duration of the end utterance silence threshold in milliseconds. | [optional]
**disable_partial_transcripts** | Option<**bool**> | Disable partial transcripts. Set to `true` to not receive partial transcripts. Defaults to `false`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


