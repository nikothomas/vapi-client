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
pub struct Say {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "exact", skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    /// This is for metadata you want to store on the task.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Say {
    pub fn new(r#type: Type, name: String) -> Say {
        Say {
            r#type,
            exact: None,
            prompt: None,
            name,
            metadata: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "say")]
    Say,
}

impl Default for Type {
    fn default() -> Type {
        Self::Say
    }
}
