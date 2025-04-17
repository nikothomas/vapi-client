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
pub struct CreateGoogleCalendarOAuth2ClientCredentialDto {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateGoogleCalendarOAuth2ClientCredentialDto {
    pub fn new(provider: Provider) -> CreateGoogleCalendarOAuth2ClientCredentialDto {
        CreateGoogleCalendarOAuth2ClientCredentialDto {
            provider,
            name: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "google.calendar.oauth2-client")]
    GooglePeriodCalendarPeriodOauth2Client,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::GooglePeriodCalendarPeriodOauth2Client
    }
}

