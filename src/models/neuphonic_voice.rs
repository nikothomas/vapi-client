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
pub struct NeuphonicVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "voiceId")]
    pub voice_id: Box<models::NeuphonicVoiceVoiceId>,
    /// This is the model that will be used. Defaults to 'neu_fast' if not specified.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    /// This is the language (ISO 639-1) that is enforced for the model.
    #[serde(rename = "language")]
    pub language: serde_json::Value,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<Box<models::ChunkPlan>>,
    /// This is the plan for voice provider fallbacks in the event that the primary voice provider fails.
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<Box<models::FallbackPlan>>,
}

impl NeuphonicVoice {
    pub fn new(provider: Provider, voice_id: models::NeuphonicVoiceVoiceId, language: serde_json::Value) -> NeuphonicVoice {
        NeuphonicVoice {
            provider,
            voice_id: Box::new(voice_id),
            model: None,
            language,
            speed: None,
            chunk_plan: None,
            fallback_plan: None,
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
/// This is the model that will be used. Defaults to 'neu_fast' if not specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "neu_hq")]
    NeuHq,
    #[serde(rename = "neu_fast")]
    NeuFast,
}

impl Default for Model {
    fn default() -> Model {
        Self::NeuHq
    }
}

