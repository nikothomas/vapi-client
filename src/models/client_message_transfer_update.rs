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
pub struct ClientMessageTransferUpdate {
    /// This is the type of the message. \"transfer-update\" is sent whenever a transfer happens.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::ClientMessageTransferUpdateDestination>,
    /// This is the assistant that the call is being transferred to. This is only sent if `destination.type` is \"assistant\".
    #[serde(rename = "toAssistant", skip_serializing_if = "Option::is_none")]
    pub to_assistant: Option<models::CreateAssistantDto>,
    /// This is the assistant that the call is being transferred from. This is only sent if `destination.type` is \"assistant\".
    #[serde(rename = "fromAssistant", skip_serializing_if = "Option::is_none")]
    pub from_assistant: Option<models::CreateAssistantDto>,
    /// This is the step that the conversation moved to.
    #[serde(rename = "toStepRecord", skip_serializing_if = "Option::is_none")]
    pub to_step_record: Option<serde_json::Value>,
    /// This is the step that the conversation moved from. =
    #[serde(rename = "fromStepRecord", skip_serializing_if = "Option::is_none")]
    pub from_step_record: Option<serde_json::Value>,
}

impl ClientMessageTransferUpdate {
    pub fn new(r#type: Type) -> ClientMessageTransferUpdate {
        ClientMessageTransferUpdate {
            r#type,
            destination: None,
            to_assistant: None,
            from_assistant: None,
            to_step_record: None,
            from_step_record: None,
        }
    }
}
/// This is the type of the message. \"transfer-update\" is sent whenever a transfer happens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Type {
    #[serde(rename = "transfer-update")]
    TransferUpdate,
}

impl Default for Type {
    fn default() -> Type {
        Self::TransferUpdate
    }
}
