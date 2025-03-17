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
pub struct ClientMessageVoiceInput {
    /// This is the type of the message. \"voice-input\" is sent when a generation is requested from voice provider.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the voice input content
    #[serde(rename = "input")]
    pub input: String,
}

impl ClientMessageVoiceInput {
    pub fn new(r#type: Type, input: String) -> ClientMessageVoiceInput {
        ClientMessageVoiceInput {
            r#type,
            input,
        }
    }
}
/// This is the type of the message. \"voice-input\" is sent when a generation is requested from voice provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "voice-input")]
    VoiceInput,
}

impl Default for Type {
    fn default() -> Type {
        Self::VoiceInput
    }
}

