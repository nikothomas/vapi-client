/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct UpdateWorkflowBlockDto {
    /// These are the pre-configured messages that will be spoken to the user while the block is running.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::ConversationBlockMessagesInner>>,
    /// This is the input schema for the block. This is the input the block needs to run. It's given to the block as `steps[0].input`  These are accessible as variables: - ({{input.propertyName}}) in context of the block execution (step) - ({{stepName.input.propertyName}}) in context of the workflow
    #[serde(rename = "inputSchema", skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<models::JsonSchema>,
    /// This is the output schema for the block. This is the output the block will return to the workflow (`{{stepName.output}}`).  These are accessible as variables: - ({{output.propertyName}}) in context of the block execution (step) - ({{stepName.output.propertyName}}) in context of the workflow (read caveat #1) - ({{blockName.output.propertyName}}) in context of the workflow (read caveat #2)  Caveats: 1. a workflow can execute a step multiple times. example, if a loop is used in the graph. {{stepName.output.propertyName}} will reference the latest usage of the step. 2. a workflow can execute a block multiple times. example, if a step is called multiple times or if a block is used in multiple steps. {{blockName.output.propertyName}} will reference the latest usage of the block. this liquid variable is just provided for convenience when creating blocks outside of a workflow with steps.
    #[serde(rename = "outputSchema", skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<models::JsonSchema>,
    /// These are the steps in the workflow.
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<models::VapiModelStepsInner>>,
    /// This is the name of the block. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateWorkflowBlockDto {
    pub fn new() -> UpdateWorkflowBlockDto {
        UpdateWorkflowBlockDto {
            messages: None,
            input_schema: None,
            output_schema: None,
            steps: None,
            name: None,
        }
    }
}
