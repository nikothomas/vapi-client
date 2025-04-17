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
pub struct ServerMessageSpeechUpdate {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ServerMessageAssistantRequestPhoneNumber>,
    /// This is the type of the message. \"speech-update\" is sent whenever assistant or user start or stop speaking.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the status of the speech update.
    #[serde(rename = "status")]
    pub status: Status,
    /// This is the role which the speech update is for.
    #[serde(rename = "role")]
    pub role: Role,
    /// This is the turn number of the speech update (0-indexed).
    #[serde(rename = "turn", skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    /// This is the timestamp of when the message was sent in milliseconds since Unix Epoch.
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
}

impl ServerMessageSpeechUpdate {
    pub fn new(r#type: Type, status: Status, role: Role) -> ServerMessageSpeechUpdate {
        ServerMessageSpeechUpdate {
            phone_number: None,
            r#type,
            status,
            role,
            turn: None,
            timestamp: None,
            artifact: None,
            assistant: None,
            customer: None,
            call: None,
        }
    }
}
/// This is the type of the message. \"speech-update\" is sent whenever assistant or user start or stop speaking.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "speech-update")]
    SpeechUpdate,
}

impl Default for Type {
    fn default() -> Type {
        Self::SpeechUpdate
    }
}
/// This is the status of the speech update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "stopped")]
    Stopped,
}

impl Default for Status {
    fn default() -> Status {
        Self::Started
    }
}
/// This is the role which the speech update is for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Assistant
    }
}

