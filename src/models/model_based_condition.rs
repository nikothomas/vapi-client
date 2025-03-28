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
pub struct ModelBasedCondition {
    /// This condition is based on a model.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the instruction which should output a boolean value when passed to a model.  You can reference any variable in the context of the current block execution (step): - \"{{output.your-property-name}}\" for current step's output - \"{{input.your-property-name}}\" for current step's input - \"{{your-step-name.output.your-property-name}}\" for another step's output (in the same workflow; read caveat #1) - \"{{your-step-name.input.your-property-name}}\" for another step's input (in the same workflow; read caveat #1) - \"{{your-block-name.output.your-property-name}}\" for another block's output (in the same workflow; read caveat #2) - \"{{your-block-name.input.your-property-name}}\" for another block's input (in the same workflow; read caveat #2) - \"{{workflow.input.your-property-name}}\" for the current workflow's input - \"{{global.your-property-name}}\" for the global context  You can also talk about the current step's output or input directly: - \"{{output.your-property-name}} is greater than 10\" - \"{{input.your-property-name}} is greater than 10\"  Examples:  - \"{{input.age}} is greater than 10\"  - \"{{input.age}} is greater than {{input.age2}}\"  - \"{{output.age}} is greater than 10\"  Caveats: 1. a workflow can execute a step multiple times. example, if a loop is used in the graph. {{stepName.input/output.propertyName}} will reference the latest usage of the step. 2. a workflow can execute a block multiple times. example, if a step is called multiple times or if a block is used in multiple steps. {{blockName.input/output.propertyName}} will reference the latest usage of the block. this liquid variable is just provided for convenience when creating blocks outside of a workflow with steps.
    #[serde(rename = "instruction")]
    pub instruction: String,
}

impl ModelBasedCondition {
    pub fn new(r#type: Type, instruction: String) -> ModelBasedCondition {
        ModelBasedCondition {
            r#type,
            instruction,
        }
    }
}
/// This condition is based on a model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "model-based")]
    ModelBased,
}

impl Default for Type {
    fn default() -> Type {
        Self::ModelBased
    }
}
