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
pub struct SayHook {
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is for metadata you want to store on the task.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "exact", skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

impl SayHook {
    pub fn new(r#type: Type) -> SayHook {
        SayHook {
            r#type,
            metadata: None,
            exact: None,
            prompt: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "say")]
    Say,
}

impl Default for Type {
    fn default() -> Type {
        Self::Say
    }
}

