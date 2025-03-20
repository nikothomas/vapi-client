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
pub struct VonageCredential {
    /// This is not returned in the API.
    #[serde(rename = "vonageApplicationPrivateKey")]
    pub vonage_application_private_key: String,
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is not returned in the API.
    #[serde(rename = "apiSecret")]
    pub api_secret: String,
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
    /// This is the Vonage Application ID for the credential.  Only relevant for Vonage credentials.
    #[serde(rename = "vonageApplicationId")]
    pub vonage_application_id: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

impl VonageCredential {
    pub fn new(
        vonage_application_private_key: String,
        provider: Provider,
        api_secret: String,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
        vonage_application_id: String,
        api_key: String,
    ) -> VonageCredential {
        VonageCredential {
            vonage_application_private_key,
            provider,
            api_secret,
            id,
            org_id,
            created_at,
            updated_at,
            vonage_application_id,
            name: None,
            api_key,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "vonage")]
    Vonage,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Vonage
    }
}
