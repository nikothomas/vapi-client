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
pub struct FallbackOpenAiTranscriber {
    /// This is the transcription provider that will be used.
    #[serde(rename = "provider")]
    pub provider: ProviderTrue,
    /// This is the model that will be used for the transcription.
    #[serde(rename = "model")]
    pub model: ModelTrue,
    /// This is the language that will be set for the transcription.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageTrue>,
}

impl FallbackOpenAiTranscriber {
    pub fn new(provider: ProviderTrue, model: ModelTrue) -> FallbackOpenAiTranscriber {
        FallbackOpenAiTranscriber {
            provider,
            model,
            language: None,
        }
    }
}
/// This is the transcription provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderTrue {
    #[serde(rename = "openai")]
    Openai,
}

impl Default for ProviderTrue {
    fn default() -> ProviderTrue {
        Self::Openai
    }
}
/// This is the model that will be used for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ModelTrue {
    #[serde(rename = "gpt-4o-transcribe")]
    Gpt4oTranscribe,
    #[serde(rename = "gpt-4o-mini-transcribe")]
    Gpt4oMiniTranscribe,
}

impl Default for ModelTrue {
    fn default() -> ModelTrue {
        Self::Gpt4oTranscribe
    }
}
/// This is the language that will be set for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LanguageTrue {
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "hy")]
    Hy,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "zh")]
    Zh,
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
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "ne")]
    Ne,
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
    #[serde(rename = "sr")]
    Sr,
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
    #[serde(rename = "tl")]
    Tl,
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
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "cy")]
    Cy,
}

impl Default for LanguageTrue {
    fn default() -> LanguageTrue {
        Self::Af
    }
}
