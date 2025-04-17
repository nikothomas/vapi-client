# FallbackVapiVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | **String** | The voices provided by Vapi | 
**speed** | Option<**f64**> | This is the speed multiplier that will be used.  @default 1 | [optional][default to 1]
**language** | Option<**String**> | This is the language code (ISO 639-1) that will be used.  @default 'en-US' | [optional][default to EnUs]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


