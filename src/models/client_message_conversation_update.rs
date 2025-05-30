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
pub struct ClientMessageConversationUpdate {
    /// This is the type of the message. \"conversation-update\" is sent when an update is committed to the conversation history.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the most up-to-date conversation history at the time the message is sent.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::ArtifactMessagesInner>>,
    /// This is the most up-to-date conversation history at the time the message is sent, formatted for OpenAI.
    #[serde(rename = "messagesOpenAIFormatted")]
    pub messages_open_ai_formatted: Vec<models::OpenAiMessage>,
}

impl ClientMessageConversationUpdate {
    pub fn new(r#type: Type, messages_open_ai_formatted: Vec<models::OpenAiMessage>) -> ClientMessageConversationUpdate {
        ClientMessageConversationUpdate {
            r#type,
            messages: None,
            messages_open_ai_formatted,
        }
    }
}
/// This is the type of the message. \"conversation-update\" is sent when an update is committed to the conversation history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "conversation-update")]
    ConversationUpdate,
}

impl Default for Type {
    fn default() -> Type {
        Self::ConversationUpdate
    }
}

