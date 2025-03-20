# HandoffStep

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block** | Option<[**models::HandoffStepBlock**](HandoffStep_block.md)> |  | [optional]
**r#type** | **String** | This is a step that takes a handoff from the previous step. This means it won't return to the calling step. The workflow execution will continue linearly.  Use case: - You want to collect information linearly (e.g. a form, provide information, etc). | 
**destinations** | Option<[**Vec<models::HandoffStepDestinationsInner>**](HandoffStep_destinations_inner.md)> | These are the destinations that the step can go to after it's done. | [optional]
**name** | **String** | This is the name of the step. | 
**block_id** | Option<**String**> | This is the id of the block to use. To use a transient block, use `block`. | [optional]
**input** | Option<[**serde_json::Value**](.md)> | This is the input to the block. You can use any key-value map as input to the block.  Example: {   \"name\": \"John Doe\",   \"age\": 20 }  You can reference any variable in the context of the current block: - \"{{your-step-name.output.your-property-name}}\" for another step's output (in the same workflow; read caveat #1) - \"{{your-step-name.input.your-property-name}}\" for another step's input (in the same workflow; read caveat #1) - \"{{your-block-name.output.your-property-name}}\" for another block's output (in the same workflow; read caveat #2) - \"{{your-block-name.input.your-property-name}}\" for another block's input (in the same workflow; read caveat #2) - \"{{workflow.input.your-property-name}}\" for the current workflow's input - \"{{global.your-property-name}}\" for the global context  Example: {   \"name\": \"{{my-tool-call-step.output.name}}\",   \"age\": \"{{my-tool-call-step.input.age}}\",   \"date\": \"{{workflow.input.date}}\" }  You can dynamically change the key name.  Example: {   \"{{my-tool-call-step.output.key-name-for-name}}\": \"{{name}}\",   \"{{my-tool-call-step.input.key-name-for-age}}\": \"{{age}}\",   \"{{workflow.input.key-name-for-date}}\": \"{{date}}\" }  You can represent the value as a string, number, boolean, array, or object.  Example: {   \"name\": \"john\",   \"age\": 20,   \"date\": \"2021-01-01\",   \"metadata\": {     \"unique-key\": \"{{my-tool-call-step.output.unique-key}}\"   },   \"array\": [\"A\", \"B\", \"C\"], }  Caveats: 1. a workflow can execute a step multiple times. example, if a loop is used in the graph. {{stepName.input/output.propertyName}} will reference the latest usage of the step. 2. a workflow can execute a block multiple times. example, if a step is called multiple times or if a block is used in multiple steps. {{blockName.input/output.propertyName}} will reference the latest usage of the block. this liquid variable is just provided for convenience when creating blocks outside of a workflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


