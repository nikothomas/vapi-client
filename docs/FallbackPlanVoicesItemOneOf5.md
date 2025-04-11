# FallbackPlanVoicesItemOneOf5

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_id** | [**models::FallbackElevenLabsVoiceId**](FallbackElevenLabsVoiceId.md) |  | 
**stability** | Option<**f64**> | Defines the stability for voice settings. | [optional]
**similarity_boost** | Option<**f64**> | Defines the similarity boost for voice settings. | [optional]
**style** | Option<**f64**> | Defines the style for voice settings. | [optional]
**use_speaker_boost** | Option<**bool**> | Defines the use speaker boost for voice settings. | [optional]
**speed** | Option<**f64**> | Defines the speed for voice settings. | [optional]
**optimize_streaming_latency** | Option<**f64**> | Defines the optimize streaming latency for voice settings. Defaults to 3. | [optional]
**enable_ssml_parsing** | Option<**bool**> | This enables the use of https://elevenlabs.io/docs/speech-synthesis/prompting#pronunciation. Defaults to false to save latency.  @default false | [optional]
**model** | Option<[**models::FallbackElevenLabsVoiceModel**](FallbackElevenLabsVoiceModel.md)> |  | [optional]
**language** | Option<**String**> | This is the language (ISO 639-1) that is enforced for the model. Currently only Turbo v2.5 supports language enforcement. For other models, an error will be returned if language code is provided. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> |  | [optional]
**provider** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


