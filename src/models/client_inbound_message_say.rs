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
pub struct ClientInboundMessageSay {
    /// This is the type of the message. Send \"say\" message to make the assistant say something.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TypeTrue>,
    /// This is the flag for whether the message should replace existing assistant speech.  @default false
    #[serde(
        rename = "interruptAssistantEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub interrupt_assistant_enabled: Option<bool>,
    /// This is the content to say.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// This is the flag to end call after content is spoken.
    #[serde(rename = "endCallAfterSpoken", skip_serializing_if = "Option::is_none")]
    pub end_call_after_spoken: Option<bool>,
    /// This is the flag for whether the message is interruptible by the user.
    #[serde(
        rename = "interruptionsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub interruptions_enabled: Option<bool>,
}

impl ClientInboundMessageSay {
    pub fn new() -> ClientInboundMessageSay {
        ClientInboundMessageSay {
            r#type: None,
            interrupt_assistant_enabled: None,
            content: None,
            end_call_after_spoken: None,
            interruptions_enabled: None,
        }
    }
}
/// This is the type of the message. Send \"say\" message to make the assistant say something.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "say")]
    Say,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Say
    }
}
