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
pub struct AssistantMessage {
    /// This is the role of the message author
    #[serde(rename = "role")]
    pub role: RoleTrue,
    /// This is the content of the assistant message
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// This is the refusal message generated by the model
    #[serde(rename = "refusal", skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// This is the tool calls generated by the model
    #[serde(rename = "tool_calls", skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<models::ToolCall>>,
    /// This is an optional name for the participant
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AssistantMessage {
    pub fn new(role: RoleTrue) -> AssistantMessage {
        AssistantMessage {
            role,
            content: None,
            refusal: None,
            tool_calls: None,
            name: None,
        }
    }
}
/// This is the role of the message author
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoleTrue {
    #[serde(rename = "assistant")]
    Assistant,
}

impl Default for RoleTrue {
    fn default() -> RoleTrue {
        Self::Assistant
    }
}
