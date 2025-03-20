# BlockStartMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contents** | Option<[**Vec<models::ToolMessageStartContentsInner>**](ToolMessageStart_contents_inner.md)> | This is an alternative to the `content` property. It allows to specify variants of the same content, one per language.  Usage: - If your assistants are multilingual, you can provide content for each language. - If you don't provide content for a language, the first item in the array will be automatically translated to the active language at that moment.  This will override the `content` property. | [optional]
**conditions** | Option<[**Vec<models::BlockStartMessageConditionsInner>**](BlockStartMessage_conditions_inner.md)> | This is an optional array of conditions that must be met for this message to be triggered. | [optional]
**r#type** | **String** | This is the message type that is triggered when the block starts. | 
**content** | Option<**String**> | This is the content that the assistant will say when this message is triggered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


