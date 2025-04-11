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
pub struct FallbackNeuphonicVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the provider-specific ID that will be used.
    #[serde(rename = "voiceId")]
    pub voice_id: String,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::FallbackNeuphonicVoiceModel>,
    /// This is the language (ISO 639-1) that is enforced for the model.
    #[serde(rename = "language")]
    pub language: std::collections::HashMap<String, serde_json::Value>,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Option<f64>>,
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
}

impl FallbackNeuphonicVoice {
    pub fn new(provider: Provider, voice_id: String, language: std::collections::HashMap<String, serde_json::Value>) -> FallbackNeuphonicVoice {
        FallbackNeuphonicVoice {
            provider,
            voice_id,
            model: None,
            language,
            speed: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "neuphonic")]
    Neuphonic,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Neuphonic
    }
}

