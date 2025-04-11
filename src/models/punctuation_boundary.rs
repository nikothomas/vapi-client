use crate::models;
use serde::{Deserialize, Serialize};

/// A punctuation enum covering various language/script punctuation marks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PunctuationBoundary {
    /// Chinese period "。"
    #[serde(rename = "。")]
    ChinesePeriod,

    /// Chinese comma "，"
    #[serde(rename = "，")]
    ChineseComma,

    /// Standard English period "."
    #[serde(rename = ".")]
    Period,

    /// Exclamation mark "!"
    #[serde(rename = "!")]
    Exclamation,

    /// Question mark "?"
    #[serde(rename = "?")]
    QuestionMark,

    /// Semicolon ";"
    #[serde(rename = ";")]
    Semicolon,

    /// Right parenthesis ")"
    #[serde(rename = ")")]
    RightParenthesis,

    /// Arabic comma "،"
    #[serde(rename = "،")]
    ArabicComma,

    /// Urdu period "۔"
    #[serde(rename = "۔")]
    UrduPeriod,

    /// Devanagari Danda "।"
    #[serde(rename = "।")]
    DevanagariDanda,

    /// Devanagari Double Danda "॥"
    #[serde(rename = "॥")]
    DevanagariDoubleDanda,

    /// Single vertical bar "|"
    #[serde(rename = "|")]
    Pipe,

    /// Double vertical bar "||"
    #[serde(rename = "||")]
    DoublePipe,

    /// Standard English comma ","
    #[serde(rename = ",")]
    Comma,

    /// Colon ":"
    #[serde(rename = ":")]
    Colon,
}

impl std::fmt::Display for PunctuationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChinesePeriod => write!(f, "。"),
            Self::ChineseComma => write!(f, "，"),
            Self::Period => write!(f, "."),
            Self::Exclamation => write!(f, "!"),
            Self::QuestionMark => write!(f, "?"),
            Self::Semicolon => write!(f, ";"),
            Self::RightParenthesis => write!(f, ")"),
            Self::ArabicComma => write!(f, "،"),
            Self::UrduPeriod => write!(f, "۔"),
            Self::DevanagariDanda => write!(f, "।"),
            Self::DevanagariDoubleDanda => write!(f, "॥"),
            Self::Pipe => write!(f, "|"),
            Self::DoublePipe => write!(f, "||"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
        }
    }
}

impl Default for PunctuationBoundary {
    fn default() -> PunctuationBoundary {
        // Choose whichever makes the most sense for your default case:
        Self::Period
    }
}
