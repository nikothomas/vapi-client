# ToolMessageDelayed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contents** | Option<[**Vec<models::ToolMessageStartContentsInner>**](ToolMessageStart_contents_inner.md)> | This is an alternative to the `content` property. It allows to specify variants of the same content, one per language.  Usage: - If your assistants are multilingual, you can provide content for each language. - If you don't provide content for a language, the first item in the array will be automatically translated to the active language at that moment.  This will override the `content` property. | [optional]
**r#type** | **String** | This message is triggered when the tool call is delayed.  There are the two things that can trigger this message: 1. The user talks with the assistant while your server is processing the request. Default is \"Sorry, a few more seconds.\" 2. The server doesn't respond within `timingMilliseconds`.  This message is never triggered for async tool calls. | 
**timing_milliseconds** | Option<**f64**> | The number of milliseconds to wait for the server response before saying this message. | [optional]
**content** | Option<**String**> | This is the content that the assistant says when this message is triggered. | [optional]
**conditions** | Option<[**Vec<models::Condition>**](Condition.md)> | This is an optional array of conditions that the tool call arguments must meet in order for this message to be triggered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


