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
pub struct NeuphonicVoice {
    /// This is the provider-specific ID that will be used.
    #[serde(rename = "voiceId")]
    pub voice_id: String,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::NeuphonicVoiceModel>,
    /// This is the language (ISO 639-1) that is enforced for the model.
    #[serde(rename = "language")]
    pub language: std::collections::HashMap<String, serde_json::Value>,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Option<f64>>,
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackPlan>,
}

impl NeuphonicVoice {
    pub fn new(voice_id: String, language: std::collections::HashMap<String, serde_json::Value>) -> NeuphonicVoice {
        NeuphonicVoice {
            voice_id,
            model: None,
            language,
            speed: None,
            chunk_plan: None,
            fallback_plan: None,
        }
    }
}

