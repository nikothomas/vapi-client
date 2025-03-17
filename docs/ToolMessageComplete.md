# ToolMessageComplete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contents** | Option<[**Vec<models::ToolMessageStartContentsInner>**](ToolMessageStart_contents_inner.md)> | This is an alternative to the `content` property. It allows to specify variants of the same content, one per language.  Usage: - If your assistants are multilingual, you can provide content for each language. - If you don't provide content for a language, the first item in the array will be automatically translated to the active language at that moment.  This will override the `content` property. | [optional]
**r#type** | **String** | This message is triggered when the tool call is complete.  This message is triggered immediately without waiting for your server to respond for async tool calls.  If this message is not provided, the model will be requested to respond.  If this message is provided, only this message will be spoken and the model will not be requested to come up with a response. It's an exclusive OR. | 
**role** | Option<**String**> | This is optional and defaults to \"assistant\".  When role=assistant, `content` is said out loud.  When role=system, `content` is passed to the model in a system message. Example:     system: default one     assistant:     user:     assistant:     user:     assistant:     user:     assistant: tool called     tool: your server response     <--- system prompt as hint     ---> model generates response which is spoken This is useful when you want to provide a hint to the model about what to say next. | [optional]
**end_call_after_spoken_enabled** | Option<**bool**> | This is an optional boolean that if true, the call will end after the message is spoken. Default is false.  This is ignored if `role` is set to `system`.  @default false | [optional]
**content** | Option<**String**> | This is the content that the assistant says when this message is triggered. | [optional]
**conditions** | Option<[**Vec<models::Condition>**](Condition.md)> | This is an optional array of conditions that the tool call arguments must meet in order for this message to be triggered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


