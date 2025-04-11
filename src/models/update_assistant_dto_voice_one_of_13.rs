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
pub struct UpdateAssistantDtoVoiceOneOf13 {
    #[serde(rename = "voiceId")]
    pub voice_id: models::VapiVoiceVoiceId,
    /// This is the speed multiplier that will be used.  @default 1
    #[serde(rename = "speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Option<f64>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::VapiVoiceLanguage>,
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackPlan>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl UpdateAssistantDtoVoiceOneOf13 {
    pub fn new(voice_id: models::VapiVoiceVoiceId, provider: Provider) -> UpdateAssistantDtoVoiceOneOf13 {
        UpdateAssistantDtoVoiceOneOf13 {
            voice_id,
            speed: None,
            language: None,
            chunk_plan: None,
            fallback_plan: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Vapi
    }
}

