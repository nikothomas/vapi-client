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
pub struct AiEdgeCondition {
    #[serde(rename = "type")]
    pub r#type: TypeTrue,
    /// This is the prompt for the AI edge condition. It should evaluate to a boolean.
    #[serde(rename = "prompt")]
    pub prompt: String,
}

impl AiEdgeCondition {
    pub fn new(r#type: TypeTrue, prompt: String) -> AiEdgeCondition {
        AiEdgeCondition { r#type, prompt }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "ai")]
    Ai,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Ai
    }
}
