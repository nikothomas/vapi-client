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
pub struct GeminiMultimodalLiveVoiceConfig {
    #[serde(rename = "prebuiltVoiceConfig")]
    pub prebuilt_voice_config: Box<models::GeminiMultimodalLivePrebuiltVoiceConfig>,
}

impl GeminiMultimodalLiveVoiceConfig {
    pub fn new(prebuilt_voice_config: models::GeminiMultimodalLivePrebuiltVoiceConfig) -> GeminiMultimodalLiveVoiceConfig {
        GeminiMultimodalLiveVoiceConfig {
            prebuilt_voice_config: Box::new(prebuilt_voice_config),
        }
    }
}

