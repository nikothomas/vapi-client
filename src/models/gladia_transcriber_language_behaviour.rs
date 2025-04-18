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

/// GladiaTranscriberLanguageBehaviour : Defines how the transcription model detects the audio language. Default value is 'automatic single language'.
/// Defines how the transcription model detects the audio language. Default value is 'automatic single language'.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GladiaTranscriberLanguageBehaviour {
    String(String),
}

impl Default for GladiaTranscriberLanguageBehaviour {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

