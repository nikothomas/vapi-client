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
pub struct ClientMessageTranscript {
    /// This is the type of the message. \"transcript\" is sent as transcriber outputs partial or final transcript.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the role for which the transcript is for.
    #[serde(rename = "role")]
    pub role: Role,
    /// This is the type of the transcript.
    #[serde(rename = "transcriptType")]
    pub transcript_type: TranscriptType,
    /// This is the transcript content.
    #[serde(rename = "transcript")]
    pub transcript: String,
}

impl ClientMessageTranscript {
    pub fn new(r#type: Type, role: Role, transcript_type: TranscriptType, transcript: String) -> ClientMessageTranscript {
        ClientMessageTranscript {
            r#type,
            role,
            transcript_type,
            transcript,
        }
    }
}
/// This is the type of the message. \"transcript\" is sent as transcriber outputs partial or final transcript.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "transcript")]
    Transcript,
    #[serde(rename = "transcript[transcriptType=\"final\"]")]
    TranscriptLeftSquareBracketTranscriptTypeEqualDoubleQuoteFinalDoubleQuoteRightSquareBracket,
}

impl Default for Type {
    fn default() -> Type {
        Self::Transcript
    }
}
/// This is the role for which the transcript is for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Assistant
    }
}
/// This is the type of the transcript.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TranscriptType {
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "final")]
    Final,
}

impl Default for TranscriptType {
    fn default() -> TranscriptType {
        Self::Partial
    }
}

