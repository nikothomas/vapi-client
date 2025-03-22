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
pub struct FallbackDeepgramVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "voiceId")]
    pub voice_id: models::DeepgramVoiceVoiceId,
    /// If set to true, this will add mip_opt_out=true as a query parameter of all API requests. See https://developers.deepgram.com/docs/the-deepgram-model-improvement-partnership-program#want-to-opt-out  This will only be used if you are using your own Deepgram API key.  @default false
    #[serde(rename = "mipOptOut", skip_serializing_if = "Option::is_none")]
    pub mip_opt_out: Option<bool>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
}

impl FallbackDeepgramVoice {
    pub fn new(
        provider: Provider,
        voice_id: models::DeepgramVoiceVoiceId,
    ) -> FallbackDeepgramVoice {
        FallbackDeepgramVoice {
            provider,
            voice_id,
            mip_opt_out: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "deepgram")]
    Deepgram,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Deepgram
    }
}
