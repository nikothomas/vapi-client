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
pub struct CreateGoogleCalendarOAuth2AuthorizationCredentialDto {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// The authorization ID for the OAuth2 authorization
    #[serde(rename = "authorizationId")]
    pub authorization_id: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateGoogleCalendarOAuth2AuthorizationCredentialDto {
    pub fn new(provider: Provider, authorization_id: String) -> CreateGoogleCalendarOAuth2AuthorizationCredentialDto {
        CreateGoogleCalendarOAuth2AuthorizationCredentialDto {
            provider,
            authorization_id,
            name: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "google.calendar.oauth2-authorization")]
    GooglePeriodCalendarPeriodOauth2Authorization,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::GooglePeriodCalendarPeriodOauth2Authorization
    }
}

