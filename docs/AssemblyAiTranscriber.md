# AssemblyAiTranscriber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the transcription provider that will be used. | 
**language** | Option<**String**> | This is the language that will be set for the transcription. | [optional]
**confidence_threshold** | Option<**f64**> | Transcripts below this confidence threshold will be discarded.  @default 0.4 | [optional]
**enable_universal_streaming_api** | Option<**bool**> | Uses Assembly AI's new Universal Streaming API. See: https://www.assemblyai.com/docs/speech-to-text/universal-streaming  @default false | [optional]
**format_turns** | Option<**bool**> | This enables formatting of transcripts. Only used when `enableUniversalStreamingApi` is true.  @default false | [optional]
**end_of_turn_confidence_threshold** | Option<**f64**> | The confidence threshold to use when determining if the end of a turn has been reached. Only used when `enableUniversalStreamingApi` is true.  @default 0.7 | [optional]
**min_end_of_turn_silence_when_confident** | Option<**f64**> | The minimum amount of silence in milliseconds required to detect end of turn when confident. Only used when `enableUniversalStreamingApi` is true.  @default 160 | [optional]
**word_finalization_max_wait_time** | Option<**f64**> | The maximum wait time for word finalization. Only used when `enableUniversalStreamingApi` is true.  @default 160 | [optional]
**max_turn_silence** | Option<**f64**> | The maximum amount of silence in milliseconds allowed in a turn before end of turn is triggered. Only used when `enableUniversalStreamingApi` is true.  @default 400 | [optional]
**realtime_url** | Option<**String**> | The WebSocket URL that the transcriber connects to. | [optional]
**word_boost** | Option<**Vec<String>**> | Add up to 2500 characters of custom vocabulary. | [optional]
**end_utterance_silence_threshold** | Option<**f64**> | The duration of the end utterance silence threshold in milliseconds. | [optional]
**disable_partial_transcripts** | Option<**bool**> | Disable partial transcripts. Set to `true` to not receive partial transcripts. Defaults to `false`. | [optional]
**fallback_plan** | Option<[**models::FallbackTranscriberPlan**](FallbackTranscriberPlan.md)> | This is the plan for voice provider fallbacks in the event that the primary voice provider fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


