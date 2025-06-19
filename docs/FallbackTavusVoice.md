# FallbackTavusVoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_enabled** | Option<**bool**> | This is the flag to toggle voice caching for the assistant. | [optional][default to true]
**provider** | **String** | This is the voice provider that will be used. | 
**voice_id** | [**models::TavusVoiceVoiceId**](TavusVoice_voiceId.md) |  | 
**persona_id** | Option<**String**> | This is the unique identifier for the persona that the replica will use in the conversation. | [optional]
**callback_url** | Option<**String**> | This is the url that will receive webhooks with updates regarding the conversation state. | [optional]
**conversation_name** | Option<**String**> | This is the name for the conversation. | [optional]
**conversational_context** | Option<**String**> | This is the context that will be appended to any context provided in the persona, if one is provided. | [optional]
**custom_greeting** | Option<**String**> | This is the custom greeting that the replica will give once a participant joines the conversation. | [optional]
**properties** | Option<[**models::TavusConversationProperties**](TavusConversationProperties.md)> | These are optional properties used to customize the conversation. | [optional]
**chunk_plan** | Option<[**models::ChunkPlan**](ChunkPlan.md)> | This is the plan for chunking the model output before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


