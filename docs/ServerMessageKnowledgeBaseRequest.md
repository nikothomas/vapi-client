# ServerMessageKnowledgeBaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageKnowledgeBaseRequestPhoneNumber**](ServerMessageKnowledgeBaseRequestPhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"knowledge-base-request\" is sent to request knowledge base documents. To enable, use `assistant.knowledgeBase.provider=custom-knowledge-base`. | 
**messages** | Option<[**Vec<models::ServerMessageKnowledgeBaseRequestMessagesItem>**](ServerMessageKnowledgeBaseRequestMessagesItem.md)> | These are the messages that are going to be sent to the `model` right after the `knowledge-base-request` webhook completes. | [optional]
**messages_open_ai_formatted** | [**Vec<models::OpenAiMessage>**](OpenAiMessage.md) | This is just `messages` formatted for OpenAI. | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


