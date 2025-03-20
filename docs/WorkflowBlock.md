# WorkflowBlock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::ConversationBlockMessagesInner>**](ConversationBlock_messages_inner.md)> | These are the pre-configured messages that will be spoken to the user while the block is running. | [optional]
**input_schema** | Option<[**models::JsonSchema**](JsonSchema.md)> | This is the input schema for the block. This is the input the block needs to run. It's given to the block as `steps[0].input`  These are accessible as variables: - ({{input.propertyName}}) in context of the block execution (step) - ({{stepName.input.propertyName}}) in context of the workflow | [optional]
**output_schema** | Option<[**models::JsonSchema**](JsonSchema.md)> | This is the output schema for the block. This is the output the block will return to the workflow (`{{stepName.output}}`).  These are accessible as variables: - ({{output.propertyName}}) in context of the block execution (step) - ({{stepName.output.propertyName}}) in context of the workflow (read caveat #1) - ({{blockName.output.propertyName}}) in context of the workflow (read caveat #2)  Caveats: 1. a workflow can execute a step multiple times. example, if a loop is used in the graph. {{stepName.output.propertyName}} will reference the latest usage of the step. 2. a workflow can execute a block multiple times. example, if a step is called multiple times or if a block is used in multiple steps. {{blockName.output.propertyName}} will reference the latest usage of the block. this liquid variable is just provided for convenience when creating blocks outside of a workflow with steps. | [optional]
**r#type** | **String** | This creates a workflow which can contain any number of steps (block executions). | 
**steps** | Option<[**Vec<models::VapiModelStepsInner>**](VapiModel_steps_inner.md)> | These are the steps in the workflow. | [optional]
**id** | **String** | This is the unique identifier for the block. | 
**org_id** | **String** | This is the unique identifier for the organization that this block belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the block was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the block was last updated. | 
**name** | Option<**String**> | This is the name of the block. This is just for your reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


