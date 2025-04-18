# FallbackRimeAiVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::RimeAiVoiceVoiceId**](RimeAIVoice_voiceId.md) |  | 
**model** | Option<**String**> | This is the model that will be used. Defaults to 'v1' when not specified. | [optional]
**speed** | Option<**f64**> | This is the speed multiplier that will be used. | [optional]
**pause_between_brackets** | Option<**bool**> | This is a flag that controls whether to add slight pauses using angle brackets. Example: “Hi. <200> I’d love to have a conversation with you.” adds a 200ms pause between the first and second sentences. | [optional]
**phonemize_between_brackets** | Option<**bool**> | This is a flag that controls whether text inside brackets should be phonemized (converted to phonetic pronunciation) - Example: \"{h'El.o} World\" will pronounce \"Hello\" as expected. | [optional]
**reduce_latency** | Option<**bool**> | This is a flag that controls whether to optimize for reduced latency in streaming. https://docs.rime.ai/api-reference/endpoint/websockets#param-reduce-latency | [optional]
**inline_speed_alpha** | Option<**String**> | This is a string that allows inline speed control using alpha notation. https://docs.rime.ai/api-reference/endpoint/websockets#param-inline-speed-alpha | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


