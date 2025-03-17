# ClientInboundMessageAddMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of the message. Send \"add-message\" message to add a message to the conversation history. | 
**message** | [**models::OpenAiMessage**](OpenAIMessage.md) | This is the message to add to the conversation. | 
**trigger_response_enabled** | Option<**bool**> | This is the flag to trigger a response, or to insert the message into the conversation history silently. Defaults to `true`.  Usage: - Use `true` to trigger a response. - Use `false` to insert the message into the conversation history silently.  @default true | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


