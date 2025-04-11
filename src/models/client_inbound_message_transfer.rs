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
pub struct ClientInboundMessageTransfer {
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::ClientInboundMessageTransferDestination>,
    /// This is the content to say.
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
}

impl ClientInboundMessageTransfer {
    pub fn new() -> ClientInboundMessageTransfer {
        ClientInboundMessageTransfer {
            destination: None,
            content: None,
        }
    }
}

