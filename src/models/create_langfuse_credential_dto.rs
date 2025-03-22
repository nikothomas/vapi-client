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
pub struct CreateLangfuseCredentialDto {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// The public key for Langfuse project. Eg: pk-lf-...
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// The secret key for Langfuse project. Eg: sk-lf-... .This is not returned in the API.
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// The host URL for Langfuse project. Eg: https://cloud.langfuse.com
    #[serde(rename = "apiUrl")]
    pub api_url: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateLangfuseCredentialDto {
    pub fn new(
        provider: Provider,
        public_key: String,
        api_key: String,
        api_url: String,
    ) -> CreateLangfuseCredentialDto {
        CreateLangfuseCredentialDto {
            provider,
            public_key,
            api_key,
            api_url,
            name: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Provider {
    #[serde(rename = "langfuse")]
    Langfuse,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Langfuse
    }
}
