# ConversationNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the Conversation node. This can be used to start a conversation with the customer.  The flow is: - Workflow starts the conversation node - Model is active with the `prompt` and global context. - Model will call a tool to exit this node. - Workflow will extract variables from the conversation. - Workflow continues. | 
**model** | Option<[**models::ConversationNodeModel**](ConversationNode_model.md)> |  | [optional]
**transcriber** | Option<[**models::ConversationNodeTranscriber**](ConversationNode_transcriber.md)> |  | [optional]
**voice** | Option<[**models::ConversationNodeVoice**](ConversationNode_voice.md)> |  | [optional]
**prompt** | Option<**String**> |  | [optional]
**global_node_plan** | Option<[**models::GlobalNodePlan**](GlobalNodePlan.md)> | This is the plan for the global node. | [optional]
**variable_extraction_plan** | Option<[**models::VariableExtractionPlan**](VariableExtractionPlan.md)> | This is the plan that controls the variable extraction from the user's response. | [optional]
**name** | **String** |  | 
**is_start** | Option<**bool**> | This is whether or not the node is the start of the workflow. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | This is for metadata you want to store on the task. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


