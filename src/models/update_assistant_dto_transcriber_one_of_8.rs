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
pub struct UpdateAssistantDtoTranscriberOneOf8 {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::GoogleTranscriberModel>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::GoogleTranscriberLanguage>,
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackTranscriberPlan>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl UpdateAssistantDtoTranscriberOneOf8 {
    pub fn new(provider: Provider) -> UpdateAssistantDtoTranscriberOneOf8 {
        UpdateAssistantDtoTranscriberOneOf8 {
            model: None,
            language: None,
            fallback_plan: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "google")]
    Google,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Google
    }
}

