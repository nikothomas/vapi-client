# ToolMessageStart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contents** | Option<[**Vec<models::TextContent>**](TextContent.md)> | This is an alternative to the `content` property. It allows to specify variants of the same content, one per language.  Usage: - If your assistants are multilingual, you can provide content for each language. - If you don't provide content for a language, the first item in the array will be automatically translated to the active language at that moment.  This will override the `content` property. | [optional]
**blocking** | Option<**bool**> | This is an optional boolean that if true, the tool call will only trigger after the message is spoken. Default is false.  @default false | [optional]
**content** | Option<**String**> | This is the content that the assistant says when this message is triggered. | [optional]
**conditions** | Option<[**Vec<models::Condition>**](Condition.md)> | This is an optional array of conditions that the tool call arguments must meet in order for this message to be triggered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


