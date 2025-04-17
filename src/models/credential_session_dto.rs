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
pub struct CredentialSessionDto {
    /// The type of credential to generate a session for. Only Nango user-facing providers are supported.
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl CredentialSessionDto {
    pub fn new(provider: Provider) -> CredentialSessionDto {
        CredentialSessionDto {
            provider,
        }
    }
}
/// The type of credential to generate a session for. Only Nango user-facing providers are supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "google.calendar.oauth2-client")]
    GooglePeriodCalendarPeriodOauth2Client,
    #[serde(rename = "google.calendar.oauth2-authorization")]
    GooglePeriodCalendarPeriodOauth2Authorization,
    #[serde(rename = "google.sheets.oauth2-authorization")]
    GooglePeriodSheetsPeriodOauth2Authorization,
    #[serde(rename = "slack.oauth2-authorization")]
    SlackPeriodOauth2Authorization,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::GooglePeriodCalendarPeriodOauth2Client
    }
}

