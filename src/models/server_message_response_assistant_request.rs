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
pub struct ServerMessageResponseAssistantRequest {
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::ServerMessageResponseAssistantRequestDestination>,
    /// This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead.
    #[serde(rename = "assistantId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<Option<String>>,
    /// This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead.  If you're unsure why you're getting an invalid assistant, try logging your response and send the JSON blob to POST /assistant which will return the validation errors.
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    /// These are the overrides for the `assistant` or `assistantId`'s settings and template variables.
    #[serde(rename = "assistantOverrides", skip_serializing_if = "Option::is_none")]
    pub assistant_overrides: Option<models::AssistantOverrides>,
    /// This is the squad that will be used for the call. To use a transient squad, use `squad` instead.
    #[serde(rename = "squadId", skip_serializing_if = "Option::is_none")]
    pub squad_id: Option<String>,
    /// This is a squad that will be used for the call. To use an existing squad, use `squadId` instead.
    #[serde(rename = "squad", skip_serializing_if = "Option::is_none")]
    pub squad: Option<models::CreateSquadDto>,
    /// This is the error if the call shouldn't be accepted. This is spoken to the customer.  If this is sent, `assistantId`, `assistant`, `squadId`, `squad`, and `destination` are ignored.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ServerMessageResponseAssistantRequest {
    pub fn new() -> ServerMessageResponseAssistantRequest {
        ServerMessageResponseAssistantRequest {
            destination: None,
            assistant_id: None,
            assistant: None,
            assistant_overrides: None,
            squad_id: None,
            squad: None,
            error: None,
        }
    }
}

