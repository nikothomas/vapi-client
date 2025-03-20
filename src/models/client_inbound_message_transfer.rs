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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientInboundMessageTransfer {
    /// This is the type of the message. Send \"transfer\" message to transfer the call to a destination.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::ClientInboundMessageTransferDestination>,
    /// This is the content to say.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl ClientInboundMessageTransfer {
    pub fn new(r#type: Type) -> ClientInboundMessageTransfer {
        ClientInboundMessageTransfer {
            r#type,
            destination: None,
            content: None,
        }
    }
}
/// This is the type of the message. Send \"transfer\" message to transfer the call to a destination.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "transfer")]
    Transfer,
}

impl Default for Type {
    fn default() -> Type {
        Self::Transfer
    }
}
