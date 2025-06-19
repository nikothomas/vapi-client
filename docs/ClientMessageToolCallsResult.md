# ClientMessageToolCallsResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ClientMessageWorkflowNodeStartedPhoneNumber**](ClientMessageWorkflowNodeStarted_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"tool-calls-result\" is sent to forward the result of a tool call to the client. | 
**timestamp** | Option<**f64**> | This is the timestamp of the message. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call that the message is associated with. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that the message is associated with. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that the message is associated with. | [optional]
**tool_call_result** | [**serde_json::Value**](.md) | This is the result of the tool call. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


