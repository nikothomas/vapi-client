# ServerMessageConversationUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageConversationUpdatePhoneNumber**](ServerMessageConversationUpdatePhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"conversation-update\" is sent when an update is committed to the conversation history. | 
**messages** | Option<[**Vec<models::ServerMessageConversationUpdateMessagesItem>**](ServerMessageConversationUpdateMessagesItem.md)> | This is the most up-to-date conversation history at the time the message is sent. | [optional]
**messages_open_ai_formatted** | [**Vec<models::OpenAiMessage>**](OpenAiMessage.md) | This is the most up-to-date conversation history at the time the message is sent, formatted for OpenAI. | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


