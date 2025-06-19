# GoHighLevelCalendarAvailabilityToolWithToolCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::CreateDtmfToolDtoMessagesInner>**](CreateDtmfToolDTO_messages_inner.md)> | These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured. | [optional]
**r#type** | **String** | The type of tool. \"gohighlevel.calendar.availability.check\" for GoHighLevel Calendar availability check tool. | 
**tool_call** | [**models::ToolCall**](ToolCall.md) |  | 
**function** | Option<[**models::OpenAiFunction**](OpenAIFunction.md)> | This is the function definition of the tool.  For `endCall`, `transferCall`, and `dtmf` tools, this is auto-filled based on tool-specific fields like `tool.destinations`. But, even in those cases, you can provide a custom function definition for advanced use cases.  An example of an advanced use case is if you want to customize the message that's spoken for `endCall` tool. You can specify a function where it returns an argument \"reason\". Then, in `messages` array, you can have many \"request-complete\" messages. One of these messages will be triggered if the `messages[].conditions` matches the \"reason\" argument. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


