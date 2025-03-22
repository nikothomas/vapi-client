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
pub struct CreatePlayHtCredentialDto {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is not returned in the API.
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreatePlayHtCredentialDto {
    pub fn new(provider: Provider, api_key: String, user_id: String) -> CreatePlayHtCredentialDto {
        CreatePlayHtCredentialDto {
            provider,
            api_key,
            user_id,
            name: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Provider {
    #[serde(rename = "playht")]
    Playht,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Playht
    }
}
