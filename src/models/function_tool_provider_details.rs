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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolProviderDetails {
    /// This is the Template URL or the Snapshot URL corresponding to the Template.
    #[serde(rename = "templateUrl", skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
    #[serde(rename = "setupInstructions", skip_serializing_if = "Option::is_none")]
    pub setup_instructions: Option<Vec<models::ToolTemplateSetup>>,
    /// The type of tool. \"function\" for Function tool.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl FunctionToolProviderDetails {
    pub fn new(r#type: Type) -> FunctionToolProviderDetails {
        FunctionToolProviderDetails {
            template_url: None,
            setup_instructions: None,
            r#type,
        }
    }
}
/// The type of tool. \"function\" for Function tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "function")]
    Function,
}

impl Default for Type {
    fn default() -> Type {
        Self::Function
    }
}
