# ClientMessageConversationUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ClientMessageWorkflowNodeStartedPhoneNumber**](ClientMessageWorkflowNodeStarted_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"conversation-update\" is sent when an update is committed to the conversation history. | 
**messages** | Option<[**Vec<models::ArtifactMessagesInner>**](Artifact_messages_inner.md)> | This is the most up-to-date conversation history at the time the message is sent. | [optional]
**messages_open_ai_formatted** | [**Vec<models::OpenAiMessage>**](OpenAIMessage.md) | This is the most up-to-date conversation history at the time the message is sent, formatted for OpenAI. | 
**timestamp** | Option<**f64**> | This is the timestamp of the message. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call that the message is associated with. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that the message is associated with. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the message is associated with. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


