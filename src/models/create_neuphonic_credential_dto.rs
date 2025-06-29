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
pub struct CreateNeuphonicCredentialDto {
    #[serde(rename = "provider")]
    pub provider: ProviderTrue,
    /// This is not returned in the API.
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateNeuphonicCredentialDto {
    pub fn new(provider: ProviderTrue, api_key: String) -> CreateNeuphonicCredentialDto {
        CreateNeuphonicCredentialDto {
            provider,
            api_key,
            name: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderTrue {
    #[serde(rename = "neuphonic")]
    Neuphonic,
}

impl Default for ProviderTrue {
    fn default() -> ProviderTrue {
        Self::Neuphonic
    }
}
