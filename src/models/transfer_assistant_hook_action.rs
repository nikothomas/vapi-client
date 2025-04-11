/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferAssistantHookAction {
    /// This is the type of action - must be \"transfer\"
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<models::TransferAssistantHookActionDestination>>,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "transfer")]
    Transfer,
}

impl Default for Type {
    fn default() -> Type {
        Self::Transfer
    }
}

