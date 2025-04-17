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
pub struct TransferDestinationSip {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<models::TransferDestinationAssistantMessage>,
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the SIP URI to transfer the call to.
    #[serde(rename = "sipUri")]
    pub sip_uri: String,
    /// This configures how transfer is executed and the experience of the destination party receiving the call. Defaults to `blind-transfer`.  @default `transferPlan.mode='blind-transfer'`
    #[serde(rename = "transferPlan", skip_serializing_if = "Option::is_none")]
    pub transfer_plan: Option<models::TransferPlan>,
    /// These are custom headers to be added to SIP refer during transfer call.
    #[serde(rename = "sipHeaders", skip_serializing_if = "Option::is_none")]
    pub sip_headers: Option<serde_json::Value>,
    /// This is the description of the destination, used by the AI to choose when and how to transfer the call.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TransferDestinationSip {
    pub fn new(r#type: Type, sip_uri: String) -> TransferDestinationSip {
        TransferDestinationSip {
            message: None,
            r#type,
            sip_uri,
            transfer_plan: None,
            sip_headers: None,
            description: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "sip")]
    Sip,
}

impl Default for Type {
    fn default() -> Type {
        Self::Sip
    }
}

