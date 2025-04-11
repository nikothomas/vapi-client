# FallbackPlayHtVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_id** | [**models::FallbackPlayHtVoiceId**](FallbackPlayHTVoiceId.md) |  | 
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**temperature** | Option<**f64**> | A floating point number between 0, exclusive, and 2, inclusive. If equal to null or not provided, the model's default temperature will be used. The temperature parameter controls variance. Lower temperatures result in more predictable results, higher temperatures allow each run to vary more, so the voice may sound less like the baseline voice. | [optional]
**emotion** | Option<[**models::FallbackPlayHtVoiceEmotion**](FallbackPlayHtVoiceEmotion.md)> |  | [optional]
**voice_guidance** | Option<**f64**> | A number between 1 and 6. Use lower numbers to reduce how unique your chosen voice will be compared to other voices. | [optional]
**style_guidance** | Option<**f64**> | A number between 1 and 30. Use lower numbers to to reduce how strong your chosen emotion will be. Higher numbers will create a very emotional performance. | [optional]
**text_guidance** | Option<**f64**> | A number between 1 and 2. This number influences how closely the generated speech adheres to the input text. Use lower values to create more fluid speech, but with a higher chance of deviating from the input text. Higher numbers will make the generated speech more accurate to the input text, ensuring that the words spoken align closely with the provided text. | [optional]
**model** | Option<[**models::FallbackPlayHtVoiceModel**](FallbackPlayHtVoiceModel.md)> |  | [optional]
**language** | Option<[**models::FallbackPlayHtVoiceLanguage**](FallbackPlayHtVoiceLanguage.md)> |  | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


