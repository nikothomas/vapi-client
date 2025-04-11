# DeepgramVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_id** | [**models::DeepgramVoiceId**](DeepgramVoiceId.md) |  | 
**mip_opt_out** | Option<**bool**> | If set to true, this will add mip_opt_out=true as a query parameter of all API requests. See https://developers.deepgram.com/docs/the-deepgram-model-improvement-partnership-program#want-to-opt-out  This will only be used if you are using your own Deepgram API key.  @default false | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> |  | [optional]
**fallback_plan** | Option<[**models::FallbackPlan**](FallbackPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


