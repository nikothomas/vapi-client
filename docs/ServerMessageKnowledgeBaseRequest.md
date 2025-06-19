# ServerMessageKnowledgeBaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ClientMessageWorkflowNodeStartedPhoneNumber**](ClientMessageWorkflowNodeStarted_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"knowledge-base-request\" is sent to request knowledge base documents. To enable, use `assistant.knowledgeBase.provider=custom-knowledge-base`. | 
**messages** | Option<[**Vec<models::ArtifactMessagesInner>**](Artifact_messages_inner.md)> | These are the messages that are going to be sent to the `model` right after the `knowledge-base-request` webhook completes. | [optional]
**messages_open_ai_formatted** | [**Vec<models::OpenAiMessage>**](OpenAIMessage.md) | This is just `messages` formatted for OpenAI. | 
**timestamp** | Option<**f64**> | This is the timestamp of the message. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the message is associated with. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that the message is associated with. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call that the message is associated with. | [optional]
**chat** | Option<[**models::Chat**](Chat.md)> | This is the chat object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


