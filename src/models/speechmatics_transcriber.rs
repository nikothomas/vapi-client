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
pub struct SpeechmaticsTranscriber {
    /// This is the transcription provider that will be used.
    #[serde(rename = "provider")]
    pub provider: ProviderTrue,
    /// This is the model that will be used for the transcription.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelTrue>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageTrue>,
    /// This is the plan for voice provider fallbacks in the event that the primary voice provider fails.
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackTranscriberPlan>,
}

impl SpeechmaticsTranscriber {
    pub fn new(provider: ProviderTrue) -> SpeechmaticsTranscriber {
        SpeechmaticsTranscriber {
            provider,
            model: None,
            language: None,
            fallback_plan: None,
        }
    }
}
/// This is the transcription provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderTrue {
    #[serde(rename = "speechmatics")]
    Speechmatics,
}

impl Default for ProviderTrue {
    fn default() -> ProviderTrue {
        Self::Speechmatics
    }
}
/// This is the model that will be used for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ModelTrue {
    #[serde(rename = "default")]
    Default,
}

impl Default for ModelTrue {
    fn default() -> ModelTrue {
        Self::Default
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LanguageTrue {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "ba")]
    Ba,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "yue")]
    Yue,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "eo")]
    Eo,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "ia")]
    Ia,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "cmn")]
    Cmn,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "mn")]
    Mn,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "ug")]
    Ug,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "cy")]
    Cy,
}

impl Default for LanguageTrue {
    fn default() -> LanguageTrue {
        Self::Auto
    }
}
