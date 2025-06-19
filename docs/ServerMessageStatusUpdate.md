# ServerMessageStatusUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ClientMessageWorkflowNodeStartedPhoneNumber**](ClientMessageWorkflowNodeStarted_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"status-update\" is sent whenever the `call.status` changes. | 
**status** | **String** | This is the status of the call. | 
**ended_reason** | Option<**String**> | This is the reason the call ended. This is only sent if the status is \"ended\". | [optional]
**messages** | Option<[**Vec<models::ArtifactMessagesInner>**](Artifact_messages_inner.md)> | These are the conversation messages of the call. This is only sent if the status is \"forwarding\". | [optional]
**messages_open_ai_formatted** | Option<[**Vec<models::OpenAiMessage>**](OpenAIMessage.md)> | These are the conversation messages of the call. This is only sent if the status is \"forwarding\". | [optional]
**destination** | Option<[**models::ServerMessageStatusUpdateDestination**](ServerMessageStatusUpdate_destination.md)> |  | [optional]
**timestamp** | Option<**f64**> | This is the timestamp of the message. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the message is associated with. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that the message is associated with. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call that the message is associated with. | [optional]
**chat** | Option<[**models::Chat**](Chat.md)> | This is the chat object. | [optional]
**transcript** | Option<**String**> | This is the transcript of the call. This is only sent if the status is \"forwarding\". | [optional]
**summary** | Option<**String**> | This is the summary of the call. This is only sent if the status is \"forwarding\". | [optional]
**inbound_phone_call_debugging_artifacts** | Option<[**serde_json::Value**](.md)> | This is the inbound phone call debugging artifacts. This is only sent if the status is \"ended\" and there was an error accepting the inbound phone call.  This will include any errors related to the \"assistant-request\" if one was made. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


