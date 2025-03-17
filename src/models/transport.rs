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
pub struct Transport {
    /// This is the provider used for the call.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// This is determines whether the assistant will have video enabled.  Only relevant for `webCall` type.
    #[serde(rename = "assistantVideoEnabled", skip_serializing_if = "Option::is_none")]
    pub assistant_video_enabled: Option<bool>,
}

impl Transport {
    pub fn new() -> Transport {
        Transport {
            provider: None,
            assistant_video_enabled: None,
        }
    }
}
/// This is the provider used for the call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "vonage")]
    Vonage,
    #[serde(rename = "vapi")]
    Vapi,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "telnyx")]
    Telnyx,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}

