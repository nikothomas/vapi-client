# ClientMessageToolCalls

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | This is the type of the message. \"tool-calls\" is sent to call a tool. | [optional]
**tool_with_tool_call_list** | [**Vec<models::ClientMessageToolCallsToolWithToolCallListInner>**](ClientMessageToolCalls_toolWithToolCallList_inner.md) | This is the list of tools calls that the model is requesting along with the original tool configuration. | 
**tool_call_list** | [**Vec<models::ToolCall>**](ToolCall.md) | This is the list of tool calls that the model is requesting. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


