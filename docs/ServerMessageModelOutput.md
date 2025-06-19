# ServerMessageModelOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ClientMessageWorkflowNodeStartedPhoneNumber**](ClientMessageWorkflowNodeStarted_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"model-output\" is sent as the model outputs tokens. | 
**timestamp** | Option<**f64**> | This is the timestamp of the message. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the message is associated with. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that the message is associated with. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call that the message is associated with. | [optional]
**chat** | Option<[**models::Chat**](Chat.md)> | This is the chat object. | [optional]
**output** | [**serde_json::Value**](.md) | This is the output of the model. It can be a token or tool call. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


