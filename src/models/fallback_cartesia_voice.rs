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
pub struct FallbackCartesiaVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// The ID of the particular voice you want to use.
    #[serde(rename = "voiceId")]
    pub voice_id: String,
    /// This is the model that will be used. This is optional and will default to the correct model for the voiceId.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    /// This is the language that will be used. This is optional and will default to the correct language for the voiceId.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// Experimental controls for Cartesia voice generation
    #[serde(rename = "experimentalControls", skip_serializing_if = "Option::is_none")]
    pub experimental_controls: Option<Box<models::CartesiaExperimentalControls>>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<Box<models::ChunkPlan>>,
}

impl FallbackCartesiaVoice {
    pub fn new(provider: Provider, voice_id: String) -> FallbackCartesiaVoice {
        FallbackCartesiaVoice {
            provider,
            voice_id,
            model: None,
            language: None,
            experimental_controls: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "cartesia")]
    Cartesia,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Cartesia
    }
}
/// This is the model that will be used. This is optional and will default to the correct model for the voiceId.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "sonic-2")]
    Sonic2,
    #[serde(rename = "sonic-english")]
    SonicEnglish,
    #[serde(rename = "sonic-multilingual")]
    SonicMultilingual,
    #[serde(rename = "sonic-preview")]
    SonicPreview,
    #[serde(rename = "sonic")]
    Sonic,
}

impl Default for Model {
    fn default() -> Model {
        Self::Sonic2
    }
}
/// This is the language that will be used. This is optional and will default to the correct language for the voiceId.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
}

impl Default for Language {
    fn default() -> Language {
        Self::En
    }
}

