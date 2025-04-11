# UpdateAssistantDtoVoiceOneOf12

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voice_id** | [**models::TavusVoiceVoiceId**](TavusVoiceVoiceId.md) |  | 
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> |  | [optional]
**persona_id** | Option<**String**> | This is the unique identifier for the persona that the replica will use in the conversation. | [optional]
**callback_url** | Option<**String**> | This is the url that will receive webhooks with updates regarding the conversation state. | [optional]
**conversation_name** | Option<**String**> | This is the name for the conversation. | [optional]
**conversational_context** | Option<**String**> | This is the context that will be appended to any context provided in the persona, if one is provided. | [optional]
**custom_greeting** | Option<**String**> | This is the custom greeting that the replica will give once a participant joines the conversation. | [optional]
**properties** | Option<[**models::TavusConversationProperties**](TavusConversationProperties.md)> |  | [optional]
**fallback_plan** | Option<[**models::FallbackPlan**](FallbackPlan.md)> |  | [optional]
**provider** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


