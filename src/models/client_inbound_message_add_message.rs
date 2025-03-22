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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ClientInboundMessageAddMessage {
    /// This is the type of the message. Send \"add-message\" message to add a message to the conversation history.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the message to add to the conversation.
    #[serde(rename = "message")]
    pub message: models::OpenAiMessage,
    /// This is the flag to trigger a response, or to insert the message into the conversation history silently. Defaults to `true`.  Usage: - Use `true` to trigger a response. - Use `false` to insert the message into the conversation history silently.  @default true
    #[serde(
        rename = "triggerResponseEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_response_enabled: Option<bool>,
}

impl ClientInboundMessageAddMessage {
    pub fn new(r#type: Type, message: models::OpenAiMessage) -> ClientInboundMessageAddMessage {
        ClientInboundMessageAddMessage {
            r#type,
            message,
            trigger_response_enabled: None,
        }
    }
}
/// This is the type of the message. Send \"add-message\" message to add a message to the conversation history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Type {
    #[serde(rename = "add-message")]
    AddMessage,
}

impl Default for Type {
    fn default() -> Type {
        Self::AddMessage
    }
}
