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
pub struct FallbackTranscriberPlanTranscribersItemOneOf9 {
    #[serde(rename = "model")]
    pub model: models::FallbackOpenAiTranscriberModel,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::FallbackOpenAiTranscriberLanguage>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl FallbackTranscriberPlanTranscribersItemOneOf9 {
    pub fn new(model: models::FallbackOpenAiTranscriberModel, provider: Provider) -> FallbackTranscriberPlanTranscribersItemOneOf9 {
        FallbackTranscriberPlanTranscribersItemOneOf9 {
            model,
            language: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "openai")]
    Openai,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Openai
    }
}

