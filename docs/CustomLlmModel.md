# CustomLlmModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::OpenAiMessage>**](OpenAIMessage.md)> | This is the starting state for the conversation. | [optional]
**tools** | Option<[**Vec<models::AnyscaleModelToolsInner>**](AnyscaleModel_tools_inner.md)> | These are the tools that the assistant can use during the call. To use existing tools, use `toolIds`.  Both `tools` and `toolIds` can be used together. | [optional]
**tool_ids** | Option<**Vec<String>**> | These are the tools that the assistant can use during the call. To use transient tools, use `tools`.  Both `tools` and `toolIds` can be used together. | [optional]
**knowledge_base** | Option<[**models::AnyscaleModelKnowledgeBase**](AnyscaleModel_knowledgeBase.md)> |  | [optional]
**knowledge_base_id** | Option<**String**> | This is the ID of the knowledge base the model will use. | [optional]
**provider** | **String** | This is the provider that will be used for the model. Any service, including your own server, that is compatible with the OpenAI API can be used. | 
**metadata_send_mode** | Option<**String**> | This determines whether metadata is sent in requests to the custom provider.  - `off` will not send any metadata. payload will look like `{ messages }` - `variable` will send `assistant.metadata` as a variable on the payload. payload will look like `{ messages, metadata }` - `destructured` will send `assistant.metadata` fields directly on the payload. payload will look like `{ messages, ...metadata }`  Further, `variable` and `destructured` will send `call`, `phoneNumber`, and `customer` objects in the payload.  Default is `variable`. | [optional]
**url** | **String** | These is the URL we'll use for the OpenAI client's `baseURL`. Ex. https://openrouter.ai/api/v1 | 
**timeout_seconds** | Option<**f64**> | This sets the timeout for the connection to the custom provider without needing to stream any tokens back. Default is 20 seconds. | [optional]
**model** | **String** | This is the name of the model. Ex. cognitivecomputations/dolphin-mixtral-8x7b | 
**temperature** | Option<**f64**> | This is the temperature that will be used for calls. Default is 0 to leverage caching for lower latency. | [optional]
**max_tokens** | Option<**f64**> | This is the max number of tokens that the assistant will be allowed to generate in each turn of the conversation. Default is 250. | [optional]
**emotion_recognition_enabled** | Option<**bool**> | This determines whether we detect user's emotion while they speak and send it as an additional info to model.  Default `false` because the model is usually are good at understanding the user's emotion from text.  @default false | [optional]
**num_fast_turns** | Option<**f64**> | This sets how many turns at the start of the conversation to use a smaller, faster model from the same provider before switching to the primary model. Example, gpt-3.5-turbo if provider is openai.  Default is 0.  @default 0 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


