# ToolsUpdateRequestOneOf8

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#async** | Option<**bool**> | This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`). | [optional]
**messages** | Option<[**Vec<models::UpdateComputerToolDtoMessagesItem>**](UpdateComputerToolDtoMessagesItem.md)> | These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured. | [optional]
**sub_type** | Option<**String**> | The sub type of tool. | [optional]
**function** | Option<[**models::OpenAiFunction**](OpenAiFunction.md)> |  | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]
**name** | Option<**String**> | The name of the tool, fixed to 'computer' | [optional]
**display_width_px** | Option<**f64**> | The display width in pixels | [optional]
**display_height_px** | Option<**f64**> | The display height in pixels | [optional]
**display_number** | Option<**f64**> | Optional display number | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


