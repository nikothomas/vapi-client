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
pub struct TransferAssistantHookAction {
    /// This is the type of action - must be \"transfer\"
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::TransferAssistantHookActionDestination>,
}

impl TransferAssistantHookAction {
    pub fn new(r#type: Type) -> TransferAssistantHookAction {
        TransferAssistantHookAction {
            r#type,
            destination: None,
        }
    }
}
/// This is the type of action - must be \"transfer\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "transfer")]
    Transfer,
}

impl Default for Type {
    fn default() -> Type {
        Self::Transfer
    }
}
