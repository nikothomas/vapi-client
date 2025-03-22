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
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct FallbackOpenAiVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the provider-specific ID that will be used. Please note that ash, ballad, coral, sage, and verse may only be used with realtime models.
    #[serde(rename = "voiceId")]
    pub voice_id: VoiceId,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
}

impl FallbackOpenAiVoice {
    pub fn new(provider: Provider, voice_id: VoiceId) -> FallbackOpenAiVoice {
        FallbackOpenAiVoice {
            provider,
            voice_id,
            speed: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "openai")]
    Openai,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Openai
    }
}
/// This is the provider-specific ID that will be used. Please note that ash, ballad, coral, sage, and verse may only be used with realtime models.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum VoiceId {
    #[serde(rename = "alloy")]
    Alloy,
    #[serde(rename = "echo")]
    Echo,
    #[serde(rename = "fable")]
    Fable,
    #[serde(rename = "onyx")]
    Onyx,
    #[serde(rename = "nova")]
    Nova,
    #[serde(rename = "shimmer")]
    Shimmer,
    #[serde(rename = "ash")]
    Ash,
    #[serde(rename = "ballad")]
    Ballad,
    #[serde(rename = "coral")]
    Coral,
    #[serde(rename = "sage")]
    Sage,
    #[serde(rename = "verse")]
    Verse,
}

impl Default for VoiceId {
    fn default() -> VoiceId {
        Self::Alloy
    }
}
