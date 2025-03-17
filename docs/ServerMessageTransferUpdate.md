# ServerMessageTransferUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageAssistantRequestPhoneNumber**](ServerMessageAssistantRequest_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"transfer-update\" is sent whenever a transfer happens. | 
**destination** | Option<[**models::ClientMessageTransferUpdateDestination**](ClientMessageTransferUpdate_destination.md)> |  | [optional]
**timestamp** | Option<**String**> | This is the ISO-8601 formatted timestamp of when the message was sent. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that is currently active. This is provided for convenience.  This matches one of the following: - `call.assistant`, - `call.assistantId`, - `call.squad[n].assistant`, - `call.squad[n].assistantId`, - `call.squadId->[n].assistant`, - `call.squadId->[n].assistantId`. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer associated with the call.  This matches one of the following: - `call.customer`, - `call.customerId`. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call object.  This matches what was returned in POST /call.  Note: This might get stale during the call. To get the latest call object, especially after the call is ended, use GET /call/:id. | [optional]
**to_assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the call is being transferred to. This is only sent if `destination.type` is \"assistant\". | [optional]
**from_assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the call is being transferred from. This is only sent if `destination.type` is \"assistant\". | [optional]
**to_step_record** | Option<[**serde_json::Value**](.md)> | This is the step that the conversation moved to. | [optional]
**from_step_record** | Option<[**serde_json::Value**](.md)> | This is the step that the conversation moved from. = | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


