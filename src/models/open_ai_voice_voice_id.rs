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

/// OpenAiVoiceVoiceId : This is the provider-specific ID that will be used. Please note that ash, ballad, coral, sage, and verse may only be used with realtime models.
/// This is the provider-specific ID that will be used. Please note that ash, ballad, coral, sage, and verse may only be used with realtime models.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiVoiceVoiceId {
    PresetVoiceOptions(String),
    OpenAiVoiceId(String),
}

impl Default for OpenAiVoiceVoiceId {
    fn default() -> Self {
        Self::PresetVoiceOptions(Default::default())
    }
}
