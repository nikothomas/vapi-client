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
pub struct ServerMessageLanguageChangeDetected {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ServerMessageAssistantRequestPhoneNumber>,
    /// This is the type of the message. \"language-change-detected\" is sent when the transcriber is automatically switched based on the detected language.
    #[serde(rename = "type")]
    pub r#type: Type,
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
    /// This is the language the transcriber is switched to.
    #[serde(rename = "language")]
    pub language: String,
}

impl ServerMessageLanguageChangeDetected {
    pub fn new(r#type: Type, language: String) -> ServerMessageLanguageChangeDetected {
        ServerMessageLanguageChangeDetected {
            phone_number: None,
            r#type,
            timestamp: None,
            artifact: None,
            assistant: None,
            customer: None,
            call: None,
            language,
        }
    }
}
/// This is the type of the message. \"language-change-detected\" is sent when the transcriber is automatically switched based on the detected language.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Type {
    #[serde(rename = "language-change-detected")]
    LanguageChangeDetected,
}

impl Default for Type {
    fn default() -> Type {
        Self::LanguageChangeDetected
    }
}
