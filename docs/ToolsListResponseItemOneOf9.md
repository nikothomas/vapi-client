# ToolsListResponseItemOneOf9

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#async** | Option<**bool**> | This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`). | [optional]
**messages** | Option<[**Vec<models::TextEditorToolMessagesItem>**](TextEditorToolMessagesItem.md)> | These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured. | [optional]
**sub_type** | **String** | The sub type of tool. | 
**id** | **String** | This is the unique identifier for the tool. | 
**org_id** | **String** | This is the unique identifier for the organization that this tool belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the tool was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the tool was last updated. | 
**function** | Option<[**models::OpenAiFunction**](OpenAiFunction.md)> |  | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]
**name** | **String** | The name of the tool, fixed to 'str_replace_editor' | 
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


