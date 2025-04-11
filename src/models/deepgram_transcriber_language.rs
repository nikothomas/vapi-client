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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeepgramTranscriberLanguage {
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "da-DK")]
    DaDk,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "de-CH")]
    DeCh,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "en-NZ")]
    EnNz,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "es-419")]
    Es419,
    #[serde(rename = "es-LATAM")]
    EsLatam,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hi-Latn")]
    HiLatn,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "ko-KR")]
    KoKr,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "multi")]
    Multi,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nl-BE")]
    NlBe,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sv-SE")]
    SvSe,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "taq")]
    Taq,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "th-TH")]
    ThTh,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
    #[serde(rename = "zh-TW")]
    ZhTw,

}

impl std::fmt::Display for DeepgramTranscriberLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bg => write!(f, "bg"),
            Self::Ca => write!(f, "ca"),
            Self::Cs => write!(f, "cs"),
            Self::Da => write!(f, "da"),
            Self::DaDk => write!(f, "da-DK"),
            Self::De => write!(f, "de"),
            Self::DeCh => write!(f, "de-CH"),
            Self::El => write!(f, "el"),
            Self::En => write!(f, "en"),
            Self::EnAu => write!(f, "en-AU"),
            Self::EnGb => write!(f, "en-GB"),
            Self::EnIn => write!(f, "en-IN"),
            Self::EnNz => write!(f, "en-NZ"),
            Self::EnUs => write!(f, "en-US"),
            Self::Es => write!(f, "es"),
            Self::Es419 => write!(f, "es-419"),
            Self::EsLatam => write!(f, "es-LATAM"),
            Self::Et => write!(f, "et"),
            Self::Fi => write!(f, "fi"),
            Self::Fr => write!(f, "fr"),
            Self::FrCa => write!(f, "fr-CA"),
            Self::Hi => write!(f, "hi"),
            Self::HiLatn => write!(f, "hi-Latn"),
            Self::Hu => write!(f, "hu"),
            Self::Id => write!(f, "id"),
            Self::It => write!(f, "it"),
            Self::Ja => write!(f, "ja"),
            Self::Ko => write!(f, "ko"),
            Self::KoKr => write!(f, "ko-KR"),
            Self::Lt => write!(f, "lt"),
            Self::Lv => write!(f, "lv"),
            Self::Ms => write!(f, "ms"),
            Self::Multi => write!(f, "multi"),
            Self::Nl => write!(f, "nl"),
            Self::NlBe => write!(f, "nl-BE"),
            Self::No => write!(f, "no"),
            Self::Pl => write!(f, "pl"),
            Self::Pt => write!(f, "pt"),
            Self::PtBr => write!(f, "pt-BR"),
            Self::Ro => write!(f, "ro"),
            Self::Ru => write!(f, "ru"),
            Self::Sk => write!(f, "sk"),
            Self::Sv => write!(f, "sv"),
            Self::SvSe => write!(f, "sv-SE"),
            Self::Ta => write!(f, "ta"),
            Self::Taq => write!(f, "taq"),
            Self::Th => write!(f, "th"),
            Self::ThTh => write!(f, "th-TH"),
            Self::Tr => write!(f, "tr"),
            Self::Uk => write!(f, "uk"),
            Self::Vi => write!(f, "vi"),
            Self::Zh => write!(f, "zh"),
            Self::ZhCn => write!(f, "zh-CN"),
            Self::ZhHans => write!(f, "zh-Hans"),
            Self::ZhHant => write!(f, "zh-Hant"),
            Self::ZhTw => write!(f, "zh-TW"),
        }
    }
}

impl Default for DeepgramTranscriberLanguage {
    fn default() -> DeepgramTranscriberLanguage {
        Self::Bg
    }
}

