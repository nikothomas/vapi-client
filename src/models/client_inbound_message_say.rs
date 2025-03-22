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
pub struct ClientInboundMessageSay {
    /// This is the type of the message. Send \"say\" message to make the assistant say something.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// This is the content to say.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// This is the flag to end call after content is spoken.
    #[serde(rename = "endCallAfterSpoken", skip_serializing_if = "Option::is_none")]
    pub end_call_after_spoken: Option<bool>,
}

impl ClientInboundMessageSay {
    pub fn new() -> ClientInboundMessageSay {
        ClientInboundMessageSay {
            r#type: None,
            content: None,
            end_call_after_spoken: None,
        }
    }
}
/// This is the type of the message. Send \"say\" message to make the assistant say something.
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
