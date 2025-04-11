/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAssistantDtoCredentialsItemOneOf36 {
    #[serde(rename = "authenticationPlan")]
    pub authentication_plan: models::OAuth2AuthenticationPlan,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl UpdateAssistantDtoCredentialsItemOneOf36 {
    pub fn new(authentication_plan: models::OAuth2AuthenticationPlan, provider: Provider) -> UpdateAssistantDtoCredentialsItemOneOf36 {
        UpdateAssistantDtoCredentialsItemOneOf36 {
            authentication_plan,
            name: None,
            provider,
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

