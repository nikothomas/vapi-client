# ToolNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the Tool node. This can be used to call a tool in your workflow.  The flow is: - Workflow starts the tool node - Model is called to extract parameters needed by the tool from the conversation history - Tool is called with the parameters - Server returns a response - Workflow continues with the response | 
**tool** | Option<[**models::ToolNodeTool**](ToolNode_tool.md)> |  | [optional]
**tool_id** | Option<**String**> | This is the tool to call. To use a transient tool, send `tool` instead. | [optional]
**name** | **String** |  | 
**is_start** | Option<**bool**> | This is whether or not the node is the start of the workflow. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | This is for metadata you want to store on the task. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


