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
pub struct ToolMessageComplete {
    /// This is an alternative to the `content` property. It allows to specify variants of the same content, one per language.  Usage: - If your assistants are multilingual, you can provide content for each language. - If you don't provide content for a language, the first item in the array will be automatically translated to the active language at that moment.  This will override the `content` property.
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<models::ToolMessageStartContentsInner>>,
    /// This message is triggered when the tool call is complete.  This message is triggered immediately without waiting for your server to respond for async tool calls.  If this message is not provided, the model will be requested to respond.  If this message is provided, only this message will be spoken and the model will not be requested to come up with a response. It's an exclusive OR.
    #[serde(rename = "type")]
    pub r#type: TypeTrue,
    /// This is optional and defaults to \"assistant\".  When role=assistant, `content` is said out loud.  When role=system, `content` is passed to the model in a system message. Example:     system: default one     assistant:     user:     assistant:     user:     assistant:     user:     assistant: tool called     tool: your server response     <--- system prompt as hint     ---> model generates response which is spoken This is useful when you want to provide a hint to the model about what to say next.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleTrue>,
    /// This is an optional boolean that if true, the call will end after the message is spoken. Default is false.  This is ignored if `role` is set to `system`.  @default false
    #[serde(
        rename = "endCallAfterSpokenEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_call_after_spoken_enabled: Option<bool>,
    /// This is the content that the assistant says when this message is triggered.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// This is an optional array of conditions that the tool call arguments must meet in order for this message to be triggered.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::Condition>>,
}

impl ToolMessageComplete {
    pub fn new(r#type: TypeTrue) -> ToolMessageComplete {
        ToolMessageComplete {
            contents: None,
            r#type,
            role: None,
            end_call_after_spoken_enabled: None,
            content: None,
            conditions: None,
        }
    }
}
/// This message is triggered when the tool call is complete.  This message is triggered immediately without waiting for your server to respond for async tool calls.  If this message is not provided, the model will be requested to respond.  If this message is provided, only this message will be spoken and the model will not be requested to come up with a response. It's an exclusive OR.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "request-complete")]
    RequestComplete,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::RequestComplete
    }
}
/// This is optional and defaults to \"assistant\".  When role=assistant, `content` is said out loud.  When role=system, `content` is passed to the model in a system message. Example:     system: default one     assistant:     user:     assistant:     user:     assistant:     user:     assistant: tool called     tool: your server response     <--- system prompt as hint     ---> model generates response which is spoken This is useful when you want to provide a hint to the model about what to say next.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoleTrue {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

impl Default for RoleTrue {
    fn default() -> RoleTrue {
        Self::Assistant
    }
}
