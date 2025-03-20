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

use crate::models;

/// ElevenLabsVoiceVoiceId : This is the provider-specific ID that will be used. Ensure the Voice is present in your 11Labs Voice Library.
/// This is the provider-specific ID that will be used. Ensure the Voice is present in your 11Labs Voice Library.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElevenLabsVoiceVoiceId {
    PresetVoiceOptions(String),
    Model11LabsVoiceId(String),
}

impl Default for ElevenLabsVoiceVoiceId {
    fn default() -> Self {
        Self::PresetVoiceOptions(Default::default())
    }
}
