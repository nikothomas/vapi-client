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
pub struct FallbackGladiaTranscriber {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::FallbackGladiaTranscriberModel>,
    #[serde(rename = "languageBehaviour", skip_serializing_if = "Option::is_none")]
    pub language_behaviour: Option<models::FallbackGladiaTranscriberLanguageBehaviour>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::FallbackGladiaTranscriberLanguage>,
    /// Provides a custom vocabulary to the model to improve accuracy of transcribing context specific words, technical terms, names, etc. If empty, this argument is ignored. ⚠️ Warning ⚠️: Please be aware that the transcription_hint field has a character limit of 600. If you provide a transcription_hint longer than 600 characters, it will be automatically truncated to meet this limit.
    #[serde(rename = "transcriptionHint", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transcription_hint: Option<Option<String>>,
    /// If prosody is true, you will get a transcription that can contain prosodies i.e. (laugh) (giggles) (malefic laugh) (toss) (music)… Default value is false.
    #[serde(rename = "prosody", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prosody: Option<Option<bool>>,
    /// If true, audio will be pre-processed to improve accuracy but latency will increase. Default value is false.
    #[serde(rename = "audioEnhancer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_enhancer: Option<Option<bool>>,
}

impl FallbackGladiaTranscriber {
    pub fn new() -> FallbackGladiaTranscriber {
        FallbackGladiaTranscriber {
            model: None,
            language_behaviour: None,
            language: None,
            transcription_hint: None,
            prosody: None,
            audio_enhancer: None,
        }
    }
}

