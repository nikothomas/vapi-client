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
pub struct HumeVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the model that will be used.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    /// The ID of the particular voice you want to use.
    #[serde(rename = "voiceId")]
    pub voice_id: String,
    /// Indicates whether the chosen voice is a preset Hume AI voice or a custom voice.
    #[serde(rename = "isCustomHumeVoice", skip_serializing_if = "Option::is_none")]
    pub is_custom_hume_voice: Option<bool>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    /// Natural language instructions describing how the synthesized speech should sound, including but not limited to tone, intonation, pacing, and accent (e.g., 'a soft, gentle voice with a strong British accent').  If a Voice is specified in the request, this description serves as acting instructions. If no Voice is specified, a new voice is generated based on this description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// This is the plan for voice provider fallbacks in the event that the primary voice provider fails.
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackPlan>,
}

impl HumeVoice {
    pub fn new(provider: Provider, voice_id: String) -> HumeVoice {
        HumeVoice {
            provider,
            model: None,
            voice_id,
            is_custom_hume_voice: None,
            chunk_plan: None,
            description: None,
            fallback_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "hume")]
    Hume,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Hume
    }
}
/// This is the model that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "octave")]
    Octave,
}

impl Default for Model {
    fn default() -> Model {
        Self::Octave
    }
}

