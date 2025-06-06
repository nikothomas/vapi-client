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
pub struct CreateTwilioCredentialDto {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is not returned in the API.
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "accountSid")]
    pub account_sid: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateTwilioCredentialDto {
    pub fn new(provider: Provider, auth_token: String, account_sid: String) -> CreateTwilioCredentialDto {
        CreateTwilioCredentialDto {
            provider,
            auth_token,
            account_sid,
            name: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}

