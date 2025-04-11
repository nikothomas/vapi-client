# ClientMessageConversationUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of the message. \"conversation-update\" is sent when an update is committed to the conversation history. | 
**messages** | Option<[**Vec<models::ClientMessageConversationUpdateMessagesItem>**](ClientMessageConversationUpdateMessagesItem.md)> | This is the most up-to-date conversation history at the time the message is sent. | [optional]
**messages_open_ai_formatted** | [**Vec<models::OpenAiMessage>**](OpenAiMessage.md) | This is the most up-to-date conversation history at the time the message is sent, formatted for OpenAI. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


