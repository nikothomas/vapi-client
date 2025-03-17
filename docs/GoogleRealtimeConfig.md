# GoogleRealtimeConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**top_p** | Option<**f64**> | This is the nucleus sampling parameter that controls the cumulative probability of tokens considered during text generation. Only applicable with the Gemini Flash 2.0 Multimodal Live API. | [optional]
**top_k** | Option<**f64**> | This is the top-k sampling parameter that limits the number of highest probability tokens considered during text generation. Only applicable with the Gemini Flash 2.0 Multimodal Live API. | [optional]
**presence_penalty** | Option<**f64**> | This is the presence penalty parameter that influences the model's likelihood to repeat information by penalizing tokens based on their presence in the text. Only applicable with the Gemini Flash 2.0 Multimodal Live API. | [optional]
**frequency_penalty** | Option<**f64**> | This is the frequency penalty parameter that influences the model's likelihood to repeat tokens by penalizing them based on their frequency in the text. Only applicable with the Gemini Flash 2.0 Multimodal Live API. | [optional]
**speech_config** | Option<[**models::GeminiMultimodalLiveSpeechConfig**](GeminiMultimodalLiveSpeechConfig.md)> | This is the speech configuration object that defines the voice settings to be used for the model's speech output. Only applicable with the Gemini Flash 2.0 Multimodal Live API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


