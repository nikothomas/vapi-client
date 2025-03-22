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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(untagged)]
pub enum ToolCallResultMessageInner {
    ToolMessageComplete(models::ToolMessageComplete),
    ToolMessageFailed(models::ToolMessageFailed),
}

impl Default for ToolCallResultMessageInner {
    fn default() -> Self {
        Self::ToolMessageComplete(Default::default())
    }
}
/// This message is triggered when the tool call is complete.  This message is triggered immediately without waiting for your server to respond for async tool calls.  If this message is not provided, the model will be requested to respond.  If this message is provided, only this message will be spoken and the model will not be requested to come up with a response. It's an exclusive OR.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "request-complete")]
    RequestComplete,
    #[serde(rename = "request-failed")]
    RequestFailed,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestComplete
    }
}
/// This is optional and defaults to \"assistant\".  When role=assistant, `content` is said out loud.  When role=system, `content` is passed to the model in a system message. Example:     system: default one     assistant:     user:     assistant:     user:     assistant:     user:     assistant: tool called     tool: your server response     <--- system prompt as hint     ---> model generates response which is spoken This is useful when you want to provide a hint to the model about what to say next.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

impl Default for Role {
    fn default() -> Role {
        Self::Assistant
    }
}
