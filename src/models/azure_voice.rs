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
pub struct AzureVoice {
    /// This is the flag to toggle voice caching for the assistant.
    #[serde(rename = "cachingEnabled", skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: ProviderTrue,
    #[serde(rename = "voiceId")]
    pub voice_id: models::AzureVoiceVoiceId,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// This is the plan for voice provider fallbacks in the event that the primary voice provider fails.
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackPlan>,
}

impl AzureVoice {
    pub fn new(provider: ProviderTrue, voice_id: models::AzureVoiceVoiceId) -> AzureVoice {
        AzureVoice {
            caching_enabled: None,
            provider,
            voice_id,
            chunk_plan: None,
            speed: None,
            fallback_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderTrue {
    #[serde(rename = "azure")]
    Azure,
}

impl Default for ProviderTrue {
    fn default() -> ProviderTrue {
        Self::Azure
    }
}
