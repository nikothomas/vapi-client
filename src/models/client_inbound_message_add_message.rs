/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientInboundMessageAddMessage {
    #[serde(rename = "message")]
    pub message: models::OpenAiMessage,
    /// This is the flag to trigger a response, or to insert the message into the conversation history silently. Defaults to `true`.  Usage: - Use `true` to trigger a response. - Use `false` to insert the message into the conversation history silently.  @default true
    #[serde(rename = "triggerResponseEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trigger_response_enabled: Option<Option<bool>>,
}

impl ClientInboundMessageAddMessage {
    pub fn new(message: models::OpenAiMessage) -> ClientInboundMessageAddMessage {
        ClientInboundMessageAddMessage {
            message,
            trigger_response_enabled: None,
        }
    }
}

