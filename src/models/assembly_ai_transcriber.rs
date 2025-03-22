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
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct AssemblyAiTranscriber {
    /// This is the transcription provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the language that will be set for the transcription.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// The WebSocket URL that the transcriber connects to.
    #[serde(rename = "realtimeUrl", skip_serializing_if = "Option::is_none")]
    pub realtime_url: Option<String>,
    /// Add up to 2500 characters of custom vocabulary.
    #[serde(rename = "wordBoost", skip_serializing_if = "Option::is_none")]
    pub word_boost: Option<Vec<String>>,
    /// The duration of the end utterance silence threshold in milliseconds.
    #[serde(
        rename = "endUtteranceSilenceThreshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_utterance_silence_threshold: Option<f64>,
    /// Disable partial transcripts. Set to `true` to not receive partial transcripts. Defaults to `false`.
    #[serde(
        rename = "disablePartialTranscripts",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_partial_transcripts: Option<bool>,
}

impl AssemblyAiTranscriber {
    pub fn new(provider: Provider) -> AssemblyAiTranscriber {
        AssemblyAiTranscriber {
            provider,
            language: None,
            realtime_url: None,
            word_boost: None,
            end_utterance_silence_threshold: None,
            disable_partial_transcripts: None,
        }
    }
}
/// This is the transcription provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Language {
    #[serde(rename = "en")]
    En,
}

impl Default for Language {
    fn default() -> Language {
        Self::En
    }
}
