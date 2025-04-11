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

/// FallbackGoogleTranscriberLanguage : This is the language that will be set for the transcription.
/// This is the language that will be set for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FallbackGoogleTranscriberLanguage {
    #[serde(rename = "Multilingual")]
    Multilingual,
    #[serde(rename = "Arabic")]
    Arabic,
    #[serde(rename = "Bengali")]
    Bengali,
    #[serde(rename = "Bulgarian")]
    Bulgarian,
    #[serde(rename = "Chinese")]
    Chinese,
    #[serde(rename = "Croatian")]
    Croatian,
    #[serde(rename = "Czech")]
    Czech,
    #[serde(rename = "Danish")]
    Danish,
    #[serde(rename = "Dutch")]
    Dutch,
    #[serde(rename = "English")]
    English,
    #[serde(rename = "Estonian")]
    Estonian,
    #[serde(rename = "Finnish")]
    Finnish,
    #[serde(rename = "French")]
    French,
    #[serde(rename = "German")]
    German,
    #[serde(rename = "Greek")]
    Greek,
    #[serde(rename = "Hebrew")]
    Hebrew,
    #[serde(rename = "Hindi")]
    Hindi,
    #[serde(rename = "Hungarian")]
    Hungarian,
    #[serde(rename = "Indonesian")]
    Indonesian,
    #[serde(rename = "Italian")]
    Italian,
    #[serde(rename = "Japanese")]
    Japanese,
    #[serde(rename = "Korean")]
    Korean,
    #[serde(rename = "Latvian")]
    Latvian,
    #[serde(rename = "Lithuanian")]
    Lithuanian,
    #[serde(rename = "Norwegian")]
    Norwegian,
    #[serde(rename = "Polish")]
    Polish,
    #[serde(rename = "Portuguese")]
    Portuguese,
    #[serde(rename = "Romanian")]
    Romanian,
    #[serde(rename = "Russian")]
    Russian,
    #[serde(rename = "Serbian")]
    Serbian,
    #[serde(rename = "Slovak")]
    Slovak,
    #[serde(rename = "Slovenian")]
    Slovenian,
    #[serde(rename = "Spanish")]
    Spanish,
    #[serde(rename = "Swahili")]
    Swahili,
    #[serde(rename = "Swedish")]
    Swedish,
    #[serde(rename = "Thai")]
    Thai,
    #[serde(rename = "Turkish")]
    Turkish,
    #[serde(rename = "Ukrainian")]
    Ukrainian,
    #[serde(rename = "Vietnamese")]
    Vietnamese,

}

impl std::fmt::Display for FallbackGoogleTranscriberLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Multilingual => write!(f, "Multilingual"),
            Self::Arabic => write!(f, "Arabic"),
            Self::Bengali => write!(f, "Bengali"),
            Self::Bulgarian => write!(f, "Bulgarian"),
            Self::Chinese => write!(f, "Chinese"),
            Self::Croatian => write!(f, "Croatian"),
            Self::Czech => write!(f, "Czech"),
            Self::Danish => write!(f, "Danish"),
            Self::Dutch => write!(f, "Dutch"),
            Self::English => write!(f, "English"),
            Self::Estonian => write!(f, "Estonian"),
            Self::Finnish => write!(f, "Finnish"),
            Self::French => write!(f, "French"),
            Self::German => write!(f, "German"),
            Self::Greek => write!(f, "Greek"),
            Self::Hebrew => write!(f, "Hebrew"),
            Self::Hindi => write!(f, "Hindi"),
            Self::Hungarian => write!(f, "Hungarian"),
            Self::Indonesian => write!(f, "Indonesian"),
            Self::Italian => write!(f, "Italian"),
            Self::Japanese => write!(f, "Japanese"),
            Self::Korean => write!(f, "Korean"),
            Self::Latvian => write!(f, "Latvian"),
            Self::Lithuanian => write!(f, "Lithuanian"),
            Self::Norwegian => write!(f, "Norwegian"),
            Self::Polish => write!(f, "Polish"),
            Self::Portuguese => write!(f, "Portuguese"),
            Self::Romanian => write!(f, "Romanian"),
            Self::Russian => write!(f, "Russian"),
            Self::Serbian => write!(f, "Serbian"),
            Self::Slovak => write!(f, "Slovak"),
            Self::Slovenian => write!(f, "Slovenian"),
            Self::Spanish => write!(f, "Spanish"),
            Self::Swahili => write!(f, "Swahili"),
            Self::Swedish => write!(f, "Swedish"),
            Self::Thai => write!(f, "Thai"),
            Self::Turkish => write!(f, "Turkish"),
            Self::Ukrainian => write!(f, "Ukrainian"),
            Self::Vietnamese => write!(f, "Vietnamese"),
        }
    }
}

impl Default for FallbackGoogleTranscriberLanguage {
    fn default() -> FallbackGoogleTranscriberLanguage {
        Self::Multilingual
    }
}

