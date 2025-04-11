/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FallbackOpenAiTranscriberLanguage : This is the language that will be set for the transcription.
/// This is the language that will be set for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FallbackOpenAiTranscriberLanguage {
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

impl std::fmt::Display for FallbackOpenAiTranscriberLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Af => write!(f, "af"),
            Self::Ar => write!(f, "ar"),
            Self::Hy => write!(f, "hy"),
            Self::Az => write!(f, "az"),
            Self::Be => write!(f, "be"),
            Self::Bs => write!(f, "bs"),
            Self::Bg => write!(f, "bg"),
            Self::Ca => write!(f, "ca"),
            Self::Zh => write!(f, "zh"),
            Self::Hr => write!(f, "hr"),
            Self::Cs => write!(f, "cs"),
            Self::Da => write!(f, "da"),
            Self::Nl => write!(f, "nl"),
            Self::En => write!(f, "en"),
            Self::Et => write!(f, "et"),
            Self::Fi => write!(f, "fi"),
            Self::Fr => write!(f, "fr"),
            Self::Gl => write!(f, "gl"),
            Self::De => write!(f, "de"),
            Self::El => write!(f, "el"),
            Self::He => write!(f, "he"),
            Self::Hi => write!(f, "hi"),
            Self::Hu => write!(f, "hu"),
            Self::Is => write!(f, "is"),
            Self::Id => write!(f, "id"),
            Self::It => write!(f, "it"),
            Self::Ja => write!(f, "ja"),
            Self::Kn => write!(f, "kn"),
            Self::Kk => write!(f, "kk"),
            Self::Ko => write!(f, "ko"),
            Self::Lv => write!(f, "lv"),
            Self::Lt => write!(f, "lt"),
            Self::Mk => write!(f, "mk"),
            Self::Ms => write!(f, "ms"),
            Self::Mr => write!(f, "mr"),
            Self::Mi => write!(f, "mi"),
            Self::Ne => write!(f, "ne"),
            Self::No => write!(f, "no"),
            Self::Fa => write!(f, "fa"),
            Self::Pl => write!(f, "pl"),
            Self::Pt => write!(f, "pt"),
            Self::Ro => write!(f, "ro"),
            Self::Ru => write!(f, "ru"),
            Self::Sr => write!(f, "sr"),
            Self::Sk => write!(f, "sk"),
            Self::Sl => write!(f, "sl"),
            Self::Es => write!(f, "es"),
            Self::Sw => write!(f, "sw"),
            Self::Sv => write!(f, "sv"),
            Self::Tl => write!(f, "tl"),
            Self::Ta => write!(f, "ta"),
            Self::Th => write!(f, "th"),
            Self::Tr => write!(f, "tr"),
            Self::Uk => write!(f, "uk"),
            Self::Ur => write!(f, "ur"),
            Self::Vi => write!(f, "vi"),
            Self::Cy => write!(f, "cy"),
        }
    }
}

impl Default for FallbackOpenAiTranscriberLanguage {
    fn default() -> FallbackOpenAiTranscriberLanguage {
        Self::Af
    }
}

