# OpenAiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::OpenAiMessage>**](OpenAIMessage.md)> | This is the starting state for the conversation. | [optional]
**tools** | Option<[**Vec<models::AnyscaleModelToolsInner>**](AnyscaleModel_tools_inner.md)> | These are the tools that the assistant can use during the call. To use existing tools, use `toolIds`.  Both `tools` and `toolIds` can be used together. | [optional]
**tool_ids** | Option<**Vec<String>**> | These are the tools that the assistant can use during the call. To use transient tools, use `tools`.  Both `tools` and `toolIds` can be used together. | [optional]
**knowledge_base** | Option<[**models::CreateCustomKnowledgeBaseDto**](CreateCustomKnowledgeBaseDTO.md)> |  | [optional]
**knowledge_base_id** | Option<**String**> | This is the ID of the knowledge base the model will use. | [optional]
**provider** | **String** | This is the provider that will be used for the model. | 
**model** | **String** | This is the OpenAI model that will be used.  When using Vapi OpenAI or your own Azure Credentials, you have the option to specify the region for the selected model. This shouldn't be specified unless you have a specific reason to do so. Vapi will automatically find the fastest region that make sense. This is helpful when you are required to comply with Data Residency rules. Learn more about Azure regions here https://azure.microsoft.com/en-us/explore/global-infrastructure/data-residency/.  @default undefined | 
**fallback_models** | Option<**Vec<String>**> | These are the fallback models that will be used if the primary model fails. This shouldn't be specified unless you have a specific reason to do so. Vapi will automatically find the fastest fallbacks that make sense. | [optional]
**tool_strict_compatibility_mode** | Option<**String**> | Azure OpenAI doesn't support `maxLength` right now https://learn.microsoft.com/en-us/azure/ai-services/openai/how-to/structured-outputs?tabs=python-secure%2Cdotnet-entra-id&pivots=programming-language-csharp#unsupported-type-specific-keywords. Need to strip.  - `strip-parameters-with-unsupported-validation` will strip parameters with unsupported validation. - `strip-unsupported-validation` will keep the parameters but strip unsupported validation.  @default `strip-unsupported-validation` | [optional]
**temperature** | Option<**f64**> | This is the temperature that will be used for calls. Default is 0 to leverage caching for lower latency. | [optional]
**max_tokens** | Option<**f64**> | This is the max number of tokens that the assistant will be allowed to generate in each turn of the conversation. Default is 250. | [optional]
**emotion_recognition_enabled** | Option<**bool**> | This determines whether we detect user's emotion while they speak and send it as an additional info to model.  Default `false` because the model is usually are good at understanding the user's emotion from text.  @default false | [optional]
**num_fast_turns** | Option<**f64**> | This sets how many turns at the start of the conversation to use a smaller, faster model from the same provider before switching to the primary model. Example, gpt-3.5-turbo if provider is openai.  Default is 0.  @default 0 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


