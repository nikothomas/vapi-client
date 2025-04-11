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

/// ClientMessageTranscriptTranscriptType : This is the type of the transcript.
/// This is the type of the transcript.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientMessageTranscriptTranscriptType {
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "final")]
    Final,

}

impl std::fmt::Display for ClientMessageTranscriptTranscriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Partial => write!(f, "partial"),
            Self::Final => write!(f, "final"),
        }
    }
}

impl Default for ClientMessageTranscriptTranscriptType {
    fn default() -> ClientMessageTranscriptTranscriptType {
        Self::Partial
    }
}

