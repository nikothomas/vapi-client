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
pub struct ServerMessagePhoneCallControl {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ServerMessagePhoneCallControlPhoneNumber>,
    /// This is the type of the message. \"phone-call-control\" is an advanced type of message.  When it is requested in `assistant.serverMessages`, the hangup and forwarding responsibilities are delegated to your server. Vapi will no longer do the actual transfer and hangup.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "request")]
    pub request: models::ServerMessagePhoneCallControlRequest,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::ServerMessagePhoneCallControlDestination>,
    /// This is the timestamp of when the message was sent in milliseconds since Unix Epoch.
    #[serde(rename = "timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Option<f64>>,
    #[serde(rename = "artifact", skip_serializing_if = "Option::is_none")]
    pub artifact: Option<models::Artifact>,
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<models::CreateCustomerDto>,
    #[serde(rename = "call", skip_serializing_if = "Option::is_none")]
    pub call: Option<models::Call>,
}

impl ServerMessagePhoneCallControl {
    pub fn new(r#type: Type, request: models::ServerMessagePhoneCallControlRequest) -> ServerMessagePhoneCallControl {
        ServerMessagePhoneCallControl {
            phone_number: None,
            r#type,
            request,
            destination: None,
            timestamp: None,
            artifact: None,
            assistant: None,
            customer: None,
            call: None,
        }
    }
}
/// This is the type of the message. \"phone-call-control\" is an advanced type of message.  When it is requested in `assistant.serverMessages`, the hangup and forwarding responsibilities are delegated to your server. Vapi will no longer do the actual transfer and hangup.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "phone-call-control")]
    PhoneCallControl,
}

impl Default for Type {
    fn default() -> Type {
        Self::PhoneCallControl
    }
}

