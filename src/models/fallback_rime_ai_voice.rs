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
pub struct FallbackRimeAiVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "voiceId")]
    pub voice_id: models::RimeAiVoiceVoiceId,
    /// This is the model that will be used. Defaults to 'v1' when not specified.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
}

impl FallbackRimeAiVoice {
    pub fn new(provider: Provider, voice_id: models::RimeAiVoiceVoiceId) -> FallbackRimeAiVoice {
        FallbackRimeAiVoice {
            provider,
            voice_id,
            model: None,
            speed: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "rime-ai")]
    RimeAi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::RimeAi
    }
}
/// This is the model that will be used. Defaults to 'v1' when not specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Model {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "mist")]
    Mist,
    #[serde(rename = "mistv2")]
    Mistv2,
}

impl Default for Model {
    fn default() -> Model {
        Self::V1
    }
}
