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

/// OpenAiVoiceModel : This is the model that will be used for text-to-speech.
/// This is the model that will be used for text-to-speech.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OpenAiVoiceModel {
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1Hd,
    #[serde(rename = "gpt-4o-mini-tts")]
    Gpt4oMiniTts,

}

impl std::fmt::Display for OpenAiVoiceModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tts1 => write!(f, "tts-1"),
            Self::Tts1Hd => write!(f, "tts-1-hd"),
            Self::Gpt4oMiniTts => write!(f, "gpt-4o-mini-tts"),
        }
    }
}

impl Default for OpenAiVoiceModel {
    fn default() -> OpenAiVoiceModel {
        Self::Tts1
    }
}

