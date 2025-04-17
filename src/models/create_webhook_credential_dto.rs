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
pub struct CreateWebhookCredentialDto {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the authentication plan. Currently supports OAuth2 RFC 6749.
    #[serde(rename = "authenticationPlan")]
    pub authentication_plan: models::OAuth2AuthenticationPlan,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateWebhookCredentialDto {
    pub fn new(provider: Provider, authentication_plan: models::OAuth2AuthenticationPlan) -> CreateWebhookCredentialDto {
        CreateWebhookCredentialDto {
            provider,
            authentication_plan,
            name: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "webhook")]
    Webhook,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Webhook
    }
}

