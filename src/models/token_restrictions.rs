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
pub struct TokenRestrictions {
    /// This determines whether the token is enabled or disabled. Default is true, it's enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// This determines the allowed origins for this token. Validates the `Origin` header. Default is any origin.  Only relevant for `public` tokens.
    #[serde(rename = "allowedOrigins", skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    /// This determines which assistantIds can be used when creating a call. Default is any assistantId.  Only relevant for `public` tokens.
    #[serde(rename = "allowedAssistantIds", skip_serializing_if = "Option::is_none")]
    pub allowed_assistant_ids: Option<Vec<String>>,
    /// This determines whether transient assistants can be used when creating a call. Default is true.  If `allowedAssistantIds` is provided, this is automatically false.  Only relevant for `public` tokens.
    #[serde(rename = "allowTransientAssistant", skip_serializing_if = "Option::is_none")]
    pub allow_transient_assistant: Option<bool>,
}

impl TokenRestrictions {
    pub fn new() -> TokenRestrictions {
        TokenRestrictions {
            enabled: None,
            allowed_origins: None,
            allowed_assistant_ids: None,
            allow_transient_assistant: None,
        }
    }
}

