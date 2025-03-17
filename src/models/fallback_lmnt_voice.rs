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
pub struct FallbackLmntVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "voiceId")]
    pub voice_id: Box<models::LmntVoiceVoiceId>,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<Box<models::ChunkPlan>>,
}

impl FallbackLmntVoice {
    pub fn new(provider: Provider, voice_id: models::LmntVoiceVoiceId) -> FallbackLmntVoice {
        FallbackLmntVoice {
            provider,
            voice_id: Box::new(voice_id),
            speed: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "lmnt")]
    Lmnt,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Lmnt
    }
}

