# ServerMessageToolCalls

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageToolCallsPhoneNumber**](ServerMessageToolCallsPhoneNumber.md)> |  | [optional]
**r#type** | Option<**String**> | This is the type of the message. \"tool-calls\" is sent to call a tool. | [optional]
**tool_with_tool_call_list** | [**Vec<models::ServerMessageToolCallsToolWithToolCallListItem>**](ServerMessageToolCallsToolWithToolCallListItem.md) | This is the list of tools calls that the model is requesting along with the original tool configuration. | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]
**tool_call_list** | [**Vec<models::ToolCall>**](ToolCall.md) | This is the list of tool calls that the model is requesting. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


