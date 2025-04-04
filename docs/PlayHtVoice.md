# PlayHtVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::PlayHtVoiceVoiceId**](PlayHTVoice_voiceId.md) |  | 
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**temperature** | Option<**f64**> | A floating point number between 0, exclusive, and 2, inclusive. If equal to null or not provided, the model's default temperature will be used. The temperature parameter controls variance. Lower temperatures result in more predictable results, higher temperatures allow each run to vary more, so the voice may sound less like the baseline voice. | [optional]
**emotion** | Option<**String**> | An emotion to be applied to the speech. | [optional]
**voice_guidance** | Option<**f64**> | A number between 1 and 6. Use lower numbers to reduce how unique your chosen voice will be compared to other voices. | [optional]
**style_guidance** | Option<**f64**> | A number between 1 and 30. Use lower numbers to to reduce how strong your chosen emotion will be. Higher numbers will create a very emotional performance. | [optional]
**text_guidance** | Option<**f64**> | A number between 1 and 2. This number influences how closely the generated speech adheres to the input text. Use lower values to create more fluid speech, but with a higher chance of deviating from the input text. Higher numbers will make the generated speech more accurate to the input text, ensuring that the words spoken align closely with the provided text. | [optional]
**model** | Option<**String**> | Playht voice model/engine to use. | [optional]
**language** | Option<**String**> | The language to use for the speech. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]
**fallback_plan** | Option<[**models::FallbackPlan**](FallbackPlan.md)> | This is the plan for voice provider fallbacks in the event that the primary voice provider fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


