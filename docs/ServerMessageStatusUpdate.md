# ServerMessageStatusUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageStatusUpdatePhoneNumber**](ServerMessageStatusUpdatePhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"status-update\" is sent whenever the `call.status` changes. | 
**status** | [**models::ServerMessageStatusUpdateStatus**](ServerMessageStatusUpdateStatus.md) |  | 
**ended_reason** | Option<[**models::ServerMessageStatusUpdateEndedReason**](ServerMessageStatusUpdateEndedReason.md)> |  | [optional]
**messages** | Option<[**Vec<models::ServerMessageStatusUpdateMessagesItem>**](ServerMessageStatusUpdateMessagesItem.md)> | These are the conversation messages of the call. This is only sent if the status is \"forwarding\". | [optional]
**messages_open_ai_formatted** | Option<[**Vec<models::OpenAiMessage>**](OpenAiMessage.md)> | These are the conversation messages of the call. This is only sent if the status is \"forwarding\". | [optional]
**destination** | Option<[**models::ServerMessageStatusUpdateDestination**](ServerMessageStatusUpdateDestination.md)> |  | [optional]
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]
**transcript** | Option<**String**> | This is the transcript of the call. This is only sent if the status is \"forwarding\". | [optional]
**summary** | Option<**String**> | This is the summary of the call. This is only sent if the status is \"forwarding\". | [optional]
**inbound_phone_call_debugging_artifacts** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is the inbound phone call debugging artifacts. This is only sent if the status is \"ended\" and there was an error accepting the inbound phone call.  This will include any errors related to the \"assistant-request\" if one was made. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


