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
pub struct ServerMessageTransferUpdate {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ServerMessageAssistantRequestPhoneNumber>,
    /// This is the type of the message. \"transfer-update\" is sent whenever a transfer happens.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::ClientMessageTransferUpdateDestination>,
    /// This is the ISO-8601 formatted timestamp of when the message was sent.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call.
    #[serde(rename = "artifact", skip_serializing_if = "Option::is_none")]
    pub artifact: Option<models::Artifact>,
    /// This is the assistant that is currently active. This is provided for convenience.  This matches one of the following: - `call.assistant`, - `call.assistantId`, - `call.squad[n].assistant`, - `call.squad[n].assistantId`, - `call.squadId->[n].assistant`, - `call.squadId->[n].assistantId`.
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    /// This is the customer associated with the call.  This matches one of the following: - `call.customer`, - `call.customerId`.
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<models::CreateCustomerDto>,
    /// This is the call object.  This matches what was returned in POST /call.  Note: This might get stale during the call. To get the latest call object, especially after the call is ended, use GET /call/:id.
    #[serde(rename = "call", skip_serializing_if = "Option::is_none")]
    pub call: Option<models::Call>,
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

impl ServerMessageTransferUpdate {
    pub fn new(r#type: Type) -> ServerMessageTransferUpdate {
        ServerMessageTransferUpdate {
            phone_number: None,
            r#type,
            destination: None,
            timestamp: None,
            artifact: None,
            assistant: None,
            customer: None,
            call: None,
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
