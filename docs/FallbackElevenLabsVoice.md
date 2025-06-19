# FallbackElevenLabsVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_enabled** | Option<**bool**> | This is the flag to toggle voice caching for the assistant. | [optional][default to true]
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::ElevenLabsVoiceVoiceId**](ElevenLabsVoice_voiceId.md) |  | 
**stability** | Option<**f64**> | Defines the stability for voice settings. | [optional]
**similarity_boost** | Option<**f64**> | Defines the similarity boost for voice settings. | [optional]
**style** | Option<**f64**> | Defines the style for voice settings. | [optional]
**use_speaker_boost** | Option<**bool**> | Defines the use speaker boost for voice settings. | [optional]
**speed** | Option<**f64**> | Defines the speed for voice settings. | [optional]
**optimize_streaming_latency** | Option<**f64**> | Defines the optimize streaming latency for voice settings. Defaults to 3. | [optional]
**enable_ssml_parsing** | Option<**bool**> | This enables the use of https://elevenlabs.io/docs/speech-synthesis/prompting#pronunciation. Defaults to false to save latency.  @default false | [optional]
**auto_mode** | Option<**bool**> | Defines the auto mode for voice settings. Defaults to false. | [optional]
**model** | Option<**String**> | This is the model that will be used. Defaults to 'eleven_turbo_v2' if not specified. | [optional]
**language** | Option<**String**> | This is the language (ISO 639-1) that is enforced for the model. Currently only Turbo v2.5 supports language enforcement. For other models, an error will be returned if language code is provided. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


