use serde::{Deserialize, Serialize};
use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChunkPlan {
    /// This determines whether the model output is chunked before being sent to the voice provider. Default `true`.
    /// Usage: - To rely on the voice provider's audio generation logic, set this to `false`.
    /// - If seeing issues with quality, set this to `true`. If disabled, Vapi-provided audio control tokens like <flush /> will not work.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This is the minimum number of characters in a chunk.
    /// Usage: - To increase quality, set this to a higher value.
    /// - To decrease latency, set this to a lower value.
    #[serde(rename = "minCharacters", skip_serializing_if = "Option::is_none")]
    pub min_characters: Option<f64>,

    /// These are the punctuations that are considered valid boundaries for a chunk to be created.
    /// Usage: - To increase quality, constrain to fewer boundaries.
    /// - To decrease latency, enable all. Default is automatically set to balance the trade-off between quality and latency based on the provider.
    #[serde(rename = "punctuationBoundaries", skip_serializing_if = "Option::is_none")]
    pub punctuation_boundaries: Option<Vec<PunctuationBoundaries>>,

    /// This is the plan for formatting the chunk before it is sent to the voice provider.
    #[serde(rename = "formatPlan", skip_serializing_if = "Option::is_none")]
    pub format_plan: Option<models::FormatPlan>,
}

impl ChunkPlan {
    pub fn new() -> ChunkPlan {
        ChunkPlan {
            enabled: None,
            min_characters: None,
            punctuation_boundaries: None,
            format_plan: None,
        }
    }
}

/// Punctuation boundaries used for chunking.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PunctuationBoundaries {
    #[serde(rename = "。")]
    FullStopChinese,
    #[serde(rename = "，")]
    CommaChinese,
    #[serde(rename = ".")]
    Period,
    #[serde(rename = "!")]
    Exclamation,
    #[serde(rename = "?")]
    QuestionMark,
    #[serde(rename = ";")]
    Semicolon,
    #[serde(rename = ")")]
    RightParenthesis,
    #[serde(rename = "،")]
    CommaArabic,
    #[serde(rename = "۔")]
    FullStopArabic,
    #[serde(rename = "।")]
    FullStopDevanagari,
    #[serde(rename = "॥")]
    DoubleVerticalBar,
    #[serde(rename = "|")]
    Pipe,
    #[serde(rename = "||")]
    DoublePipe,
    #[serde(rename = ",")]
    Comma,
    #[serde(rename = ":")]
    Colon,
}

impl Default for PunctuationBoundaries {
    fn default() -> PunctuationBoundaries {
        PunctuationBoundaries::Period // Default value for chunking
    }
}
