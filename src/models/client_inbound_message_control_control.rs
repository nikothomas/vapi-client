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

/// ClientInboundMessageControlControl : This is the control action
/// This is the control action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientInboundMessageControlControl {
    #[serde(rename = "mute-assistant")]
    MuteAssistant,
    #[serde(rename = "unmute-assistant")]
    UnmuteAssistant,
    #[serde(rename = "say-first-message")]
    SayFirstMessage,

}

impl std::fmt::Display for ClientInboundMessageControlControl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MuteAssistant => write!(f, "mute-assistant"),
            Self::UnmuteAssistant => write!(f, "unmute-assistant"),
            Self::SayFirstMessage => write!(f, "say-first-message"),
        }
    }
}

impl Default for ClientInboundMessageControlControl {
    fn default() -> ClientInboundMessageControlControl {
        Self::MuteAssistant
    }
}

