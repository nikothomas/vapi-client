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
pub struct FallbackPlanVoicesItemOneOf4 {
    #[serde(rename = "voiceId")]
    pub voice_id: models::FallbackDeepgramVoiceId,
    /// If set to true, this will add mip_opt_out=true as a query parameter of all API requests. See https://developers.deepgram.com/docs/the-deepgram-model-improvement-partnership-program#want-to-opt-out  This will only be used if you are using your own Deepgram API key.  @default false
    #[serde(rename = "mipOptOut", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mip_opt_out: Option<Option<bool>>,
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl FallbackPlanVoicesItemOneOf4 {
    pub fn new(voice_id: models::FallbackDeepgramVoiceId, provider: Provider) -> FallbackPlanVoicesItemOneOf4 {
        FallbackPlanVoicesItemOneOf4 {
            voice_id,
            mip_opt_out: None,
            chunk_plan: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "deepgram")]
    Deepgram,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Deepgram
    }
}

