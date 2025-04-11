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
pub struct FallbackTranscriberPlanTranscribersItemOneOf7 {
    /// This is the model that will be used for the transcription.
    #[serde(rename = "model", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub model: Option<Option<Model>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::FallbackTalkscriberTranscriberLanguage>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl FallbackTranscriberPlanTranscribersItemOneOf7 {
    pub fn new(provider: Provider) -> FallbackTranscriberPlanTranscribersItemOneOf7 {
        FallbackTranscriberPlanTranscribersItemOneOf7 {
            model: None,
            language: None,
            provider,
        }
    }
}
/// This is the model that will be used for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "whisper")]
    Whisper,
}

impl Default for Model {
    fn default() -> Model {
        Self::Whisper
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "talkscriber")]
    Talkscriber,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Talkscriber
    }
}

