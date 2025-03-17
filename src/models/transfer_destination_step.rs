/*
 * Vapi API
 *
 * Voice AI for developers.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferDestinationStep {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::TransferDestinationAssistantMessage>>,
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the step to transfer to.
    #[serde(rename = "stepName")]
    pub step_name: String,
    /// This is the description of the destination, used by the AI to choose when and how to transfer the call.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TransferDestinationStep {
    pub fn new(r#type: Type, step_name: String) -> TransferDestinationStep {
        TransferDestinationStep {
            message: None,
            r#type,
            step_name,
            description: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "step")]
    Step,
}

impl Default for Type {
    fn default() -> Type {
        Self::Step
    }
}

