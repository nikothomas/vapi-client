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
pub struct ServerMessageTransferDestinationRequest {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ServerMessageTransferDestinationRequestPhoneNumber>,
    /// This is the type of the message. \"transfer-destination-request\" is sent when the model is requesting transfer but destination is unknown.
    #[serde(rename = "type")]
    pub r#type: Type,
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

impl ServerMessageTransferDestinationRequest {
    pub fn new(r#type: Type) -> ServerMessageTransferDestinationRequest {
        ServerMessageTransferDestinationRequest {
            phone_number: None,
            r#type,
            timestamp: None,
            artifact: None,
            assistant: None,
            customer: None,
            call: None,
        }
    }
}
/// This is the type of the message. \"transfer-destination-request\" is sent when the model is requesting transfer but destination is unknown.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "transfer-destination-request")]
    TransferDestinationRequest,
}

impl Default for Type {
    fn default() -> Type {
        Self::TransferDestinationRequest
    }
}

