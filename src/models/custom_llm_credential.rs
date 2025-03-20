/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomLlmCredential {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is not returned in the API.
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// This is the authentication plan. Currently supports OAuth2 RFC 6749. To use Bearer authentication, use apiKey
    #[serde(rename = "authenticationPlan", skip_serializing_if = "Option::is_none")]
    pub authentication_plan: Option<models::OAuth2AuthenticationPlan>,
    /// This is the unique identifier for the credential.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the org that this credential belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the credential was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the assistant was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the authentication session for the credential. Available for credentials that have an authentication plan.
    #[serde(
        rename = "authenticationSession",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_session: Option<models::Oauth2AuthenticationSession>,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CustomLlmCredential {
    pub fn new(
        provider: Provider,
        api_key: String,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
    ) -> CustomLlmCredential {
        CustomLlmCredential {
            provider,
            api_key,
            authentication_plan: None,
            id,
            org_id,
            created_at,
            updated_at,
            authentication_session: None,
            name: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "custom-llm")]
    CustomLlm,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::CustomLlm
    }
}
