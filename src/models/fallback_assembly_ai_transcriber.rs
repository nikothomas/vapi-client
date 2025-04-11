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
pub struct FallbackAssemblyAiTranscriber {
    /// This is the language that will be set for the transcription.
    #[serde(rename = "language", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<Language>>,
    /// The WebSocket URL that the transcriber connects to.
    #[serde(rename = "realtimeUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub realtime_url: Option<Option<String>>,
    /// Add up to 2500 characters of custom vocabulary.
    #[serde(rename = "wordBoost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub word_boost: Option<Option<Vec<String>>>,
    /// The duration of the end utterance silence threshold in milliseconds.
    #[serde(rename = "endUtteranceSilenceThreshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_utterance_silence_threshold: Option<Option<f64>>,
    /// Disable partial transcripts. Set to `true` to not receive partial transcripts. Defaults to `false`.
    #[serde(rename = "disablePartialTranscripts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disable_partial_transcripts: Option<Option<bool>>,
}

impl FallbackAssemblyAiTranscriber {
    pub fn new() -> FallbackAssemblyAiTranscriber {
        FallbackAssemblyAiTranscriber {
            language: None,
            realtime_url: None,
            word_boost: None,
            end_utterance_silence_threshold: None,
            disable_partial_transcripts: None,
        }
    }
}
/// This is the language that will be set for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    En,
}

impl Default for Language {
    fn default() -> Language {
        Self::En
    }
}

