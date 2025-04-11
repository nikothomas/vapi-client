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

/// PlayHtVoiceModel : Playht voice model/engine to use.
/// Playht voice model/engine to use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayHtVoiceModel {
    #[serde(rename = "PlayHT2.0")]
    PlayHt2Period0,
    #[serde(rename = "PlayHT2.0-turbo")]
    PlayHt2Period0Turbo,
    #[serde(rename = "Play3.0-mini")]
    Play3Period0Mini,
    #[serde(rename = "PlayDialog")]
    PlayDialog,

}

impl std::fmt::Display for PlayHtVoiceModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PlayHt2Period0 => write!(f, "PlayHT2.0"),
            Self::PlayHt2Period0Turbo => write!(f, "PlayHT2.0-turbo"),
            Self::Play3Period0Mini => write!(f, "Play3.0-mini"),
            Self::PlayDialog => write!(f, "PlayDialog"),
        }
    }
}

impl Default for PlayHtVoiceModel {
    fn default() -> PlayHtVoiceModel {
        Self::PlayHt2Period0
    }
}

