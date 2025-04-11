# ClientMessageTransferUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of the message. \"transfer-update\" is sent whenever a transfer happens. | 
**destination** | Option<[**models::ClientMessageTransferUpdateDestination**](ClientMessageTransferUpdateDestination.md)> |  | [optional]
**to_assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**from_assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**to_step_record** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is the step that the conversation moved to. | [optional]
**from_step_record** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is the step that the conversation moved from. = | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


