# ClientMessageTransferUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ClientMessageWorkflowNodeStartedPhoneNumber**](ClientMessageWorkflowNodeStarted_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"transfer-update\" is sent whenever a transfer happens. | 
**destination** | Option<[**models::ClientMessageTransferUpdateDestination**](ClientMessageTransferUpdate_destination.md)> |  | [optional]
**timestamp** | Option<**f64**> | This is the timestamp of the message. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call that the message is associated with. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that the message is associated with. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the message is associated with. | [optional]
**to_assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the call is being transferred to. This is only sent if `destination.type` is \"assistant\". | [optional]
**from_assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the call is being transferred from. This is only sent if `destination.type` is \"assistant\". | [optional]
**to_step_record** | Option<[**serde_json::Value**](.md)> | This is the step that the conversation moved to. | [optional]
**from_step_record** | Option<[**serde_json::Value**](.md)> | This is the step that the conversation moved from. = | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


