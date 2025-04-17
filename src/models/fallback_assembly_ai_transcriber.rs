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
pub struct FallbackAssemblyAiTranscriber {
    /// This is the transcription provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the language that will be set for the transcription.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// Transcripts below this confidence threshold will be discarded.  @default 0.4
    #[serde(rename = "confidenceThreshold", skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    /// The WebSocket URL that the transcriber connects to.
    #[serde(rename = "realtimeUrl", skip_serializing_if = "Option::is_none")]
    pub realtime_url: Option<String>,
    /// Add up to 2500 characters of custom vocabulary.
    #[serde(rename = "wordBoost", skip_serializing_if = "Option::is_none")]
    pub word_boost: Option<Vec<String>>,
    /// The duration of the end utterance silence threshold in milliseconds.
    #[serde(rename = "endUtteranceSilenceThreshold", skip_serializing_if = "Option::is_none")]
    pub end_utterance_silence_threshold: Option<f64>,
    /// Disable partial transcripts. Set to `true` to not receive partial transcripts. Defaults to `false`.
    #[serde(rename = "disablePartialTranscripts", skip_serializing_if = "Option::is_none")]
    pub disable_partial_transcripts: Option<bool>,
}

impl FallbackAssemblyAiTranscriber {
    pub fn new(provider: Provider) -> FallbackAssemblyAiTranscriber {
        FallbackAssemblyAiTranscriber {
            provider,
            language: None,
            confidence_threshold: None,
            realtime_url: None,
            word_boost: None,
            end_utterance_silence_threshold: None,
            disable_partial_transcripts: None,
        }
    }
}
/// This is the transcription provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "assembly-ai")]
    AssemblyAi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::AssemblyAi
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

