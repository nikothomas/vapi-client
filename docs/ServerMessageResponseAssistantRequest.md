# ServerMessageResponseAssistantRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<[**models::ServerMessageResponseAssistantRequestDestination**](ServerMessageResponseAssistantRequest_destination.md)> |  | [optional]
**assistant_id** | Option<**String**> | This is the assistant ID that will be used for the call. To use a transient assistant, use `assistant` instead.  To start a call with: - Assistant, use `assistantId` or `assistant` - Squad, use `squadId` or `squad` - Workflow, use `workflowId` or `workflow` | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead.  To start a call with: - Assistant, use `assistant` - Squad, use `squad` - Workflow, use `workflow` | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | These are the overrides for the `assistant` or `assistantId`'s settings and template variables. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for the call. To use a transient squad, use `squad` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**squad** | Option<[**models::CreateSquadDto**](CreateSquadDTO.md)> | This is a squad that will be used for the call. To use an existing squad, use `squadId` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**workflow_id** | Option<**String**> | This is the workflow that will be used for the call. To use a transient workflow, use `workflow` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**workflow** | Option<[**models::CreateWorkflowDto**](CreateWorkflowDTO.md)> | This is a workflow that will be used for the call. To use an existing workflow, use `workflowId` instead.  To start a call with: - Assistant, use `assistant` or `assistantId` - Squad, use `squad` or `squadId` - Workflow, use `workflow` or `workflowId` | [optional]
**workflow_overrides** | Option<[**models::WorkflowOverrides**](WorkflowOverrides.md)> | These are the overrides for the `workflow` or `workflowId`'s settings and template variables. | [optional]
**error** | Option<**String**> | This is the error if the call shouldn't be accepted. This is spoken to the customer.  If this is sent, `assistantId`, `assistant`, `squadId`, `squad`, and `destination` are ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


