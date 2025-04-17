# FallbackCartesiaVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | **String** | The ID of the particular voice you want to use. | 
**model** | Option<**String**> | This is the model that will be used. This is optional and will default to the correct model for the voiceId. | [optional]
**language** | Option<**String**> | This is the language that will be used. This is optional and will default to the correct language for the voiceId. | [optional]
**experimental_controls** | Option<[**models::CartesiaExperimentalControls**](CartesiaExperimentalControls.md)> | Experimental controls for Cartesia voice generation | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


