# UpdateApiRequestToolDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::CreateDtmfToolDtoMessagesInner>**](CreateDtmfToolDTO_messages_inner.md)> | These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured. | [optional]
**method** | Option<**String**> |  | [optional]
**timeout_seconds** | Option<**f64**> | This is the timeout in seconds for the request. Defaults to 20 seconds.  @default 20 | [optional]
**function** | Option<[**models::OpenAiFunction**](OpenAIFunction.md)> | This is the function definition of the tool.  For `endCall`, `transferCall`, and `dtmf` tools, this is auto-filled based on tool-specific fields like `tool.destinations`. But, even in those cases, you can provide a custom function definition for advanced use cases.  An example of an advanced use case is if you want to customize the message that's spoken for `endCall` tool. You can specify a function where it returns an argument \"reason\". Then, in `messages` array, you can have many \"request-complete\" messages. One of these messages will be triggered if the `messages[].conditions` matches the \"reason\" argument. | [optional]
**name** | Option<**String**> | This is the name of the tool. This will be passed to the model.  Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 40. | [optional]
**description** | Option<**String**> | This is the description of the tool. This will be passed to the model. | [optional]
**url** | Option<**String**> | This is where the request will be sent. | [optional]
**body** | Option<[**models::JsonSchema**](JsonSchema.md)> | This is the body of the request. | [optional]
**headers** | Option<[**models::JsonSchema**](JsonSchema.md)> | These are the headers to send in the request. | [optional]
**backoff_plan** | Option<[**models::BackoffPlan**](BackoffPlan.md)> | This is the backoff plan if the request fails. Defaults to undefined (the request will not be retried).  @default undefined (the request will not be retried) | [optional]
**variable_extraction_plan** | Option<[**models::VariableExtractionPlan**](VariableExtractionPlan.md)> | This is the plan that controls the variable extraction from the tool's response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


