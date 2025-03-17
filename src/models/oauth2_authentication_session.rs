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
pub struct Oauth2AuthenticationSession {
    /// This is the OAuth2 access token.
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// This is the OAuth2 access token expiration.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl Oauth2AuthenticationSession {
    pub fn new() -> Oauth2AuthenticationSession {
        Oauth2AuthenticationSession {
            access_token: None,
            expires_at: None,
        }
    }
}

