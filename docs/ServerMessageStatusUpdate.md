# ServerMessageStatusUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageAssistantRequestPhoneNumber**](ServerMessageAssistantRequest_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"status-update\" is sent whenever the `call.status` changes. | 
**status** | **String** | This is the status of the call. | 
**ended_reason** | Option<**String**> | This is the reason the call ended. This is only sent if the status is \"ended\". | [optional]
**messages** | Option<[**Vec<models::ArtifactMessagesInner>**](Artifact_messages_inner.md)> | These are the conversation messages of the call. This is only sent if the status is \"forwarding\". | [optional]
**messages_open_ai_formatted** | Option<[**Vec<models::OpenAiMessage>**](OpenAIMessage.md)> | These are the conversation messages of the call. This is only sent if the status is \"forwarding\". | [optional]
**destination** | Option<[**models::ServerMessageStatusUpdateDestination**](ServerMessageStatusUpdate_destination.md)> |  | [optional]
**timestamp** | Option<**String**> | This is the ISO-8601 formatted timestamp of when the message was sent. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that is currently active. This is provided for convenience.  This matches one of the following: - `call.assistant`, - `call.assistantId`, - `call.squad[n].assistant`, - `call.squad[n].assistantId`, - `call.squadId->[n].assistant`, - `call.squadId->[n].assistantId`. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer associated with the call.  This matches one of the following: - `call.customer`, - `call.customerId`. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call object.  This matches what was returned in POST /call.  Note: This might get stale during the call. To get the latest call object, especially after the call is ended, use GET /call/:id. | [optional]
**transcript** | Option<**String**> | This is the transcript of the call. This is only sent if the status is \"forwarding\". | [optional]
**summary** | Option<**String**> | This is the summary of the call. This is only sent if the status is \"forwarding\". | [optional]
**inbound_phone_call_debugging_artifacts** | Option<[**serde_json::Value**](.md)> | This is the inbound phone call debugging artifacts. This is only sent if the status is \"ended\" and there was an error accepting the inbound phone call.  This will include any errors related to the \"assistant-request\" if one was made. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


