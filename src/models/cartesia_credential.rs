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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CartesiaCredential {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is not returned in the API.
    #[serde(rename = "apiKey")]
    pub api_key: String,
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
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CartesiaCredential {
    pub fn new(
        provider: Provider,
        api_key: String,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
    ) -> CartesiaCredential {
        CartesiaCredential {
            provider,
            api_key,
            id,
            org_id,
            created_at,
            updated_at,
            name: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Provider {
    #[serde(rename = "cartesia")]
    Cartesia,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Cartesia
    }
}
