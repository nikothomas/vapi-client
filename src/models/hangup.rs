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
pub struct Hangup {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "name")]
    pub name: String,
    /// This is for metadata you want to store on the task.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Hangup {
    pub fn new(r#type: Type, name: String) -> Hangup {
        Hangup {
            r#type,
            name,
            metadata: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "hangup")]
    Hangup,
}

impl Default for Type {
    fn default() -> Type {
        Self::Hangup
    }
}

