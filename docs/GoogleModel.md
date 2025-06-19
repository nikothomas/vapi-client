# GoogleModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::OpenAiMessage>**](OpenAIMessage.md)> | This is the starting state for the conversation. | [optional]
**tools** | Option<[**Vec<models::AnyscaleModelToolsInner>**](AnyscaleModel_tools_inner.md)> | These are the tools that the assistant can use during the call. To use existing tools, use `toolIds`.  Both `tools` and `toolIds` can be used together. | [optional]
**tool_ids** | Option<**Vec<String>**> | These are the tools that the assistant can use during the call. To use transient tools, use `tools`.  Both `tools` and `toolIds` can be used together. | [optional]
**knowledge_base** | Option<[**models::CreateCustomKnowledgeBaseDto**](CreateCustomKnowledgeBaseDTO.md)> |  | [optional]
**knowledge_base_id** | Option<**String**> | This is the ID of the knowledge base the model will use. | [optional]
**model** | **String** | This is the Google model that will be used. | 
**provider** | **String** |  | 
**realtime_config** | Option<[**models::GoogleRealtimeConfig**](GoogleRealtimeConfig.md)> | This is the session configuration for the Gemini Flash 2.0 Multimodal Live API. Only applicable if the model `gemini-2.0-flash-realtime-exp` is selected. | [optional]
**temperature** | Option<**f64**> | This is the temperature that will be used for calls. Default is 0 to leverage caching for lower latency. | [optional]
**max_tokens** | Option<**f64**> | This is the max number of tokens that the assistant will be allowed to generate in each turn of the conversation. Default is 250. | [optional]
**emotion_recognition_enabled** | Option<**bool**> | This determines whether we detect user's emotion while they speak and send it as an additional info to model.  Default `false` because the model is usually are good at understanding the user's emotion from text.  @default false | [optional]
**num_fast_turns** | Option<**f64**> | This sets how many turns at the start of the conversation to use a smaller, faster model from the same provider before switching to the primary model. Example, gpt-3.5-turbo if provider is openai.  Default is 0.  @default 0 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


