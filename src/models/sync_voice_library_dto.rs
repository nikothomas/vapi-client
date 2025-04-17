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
pub struct SyncVoiceLibraryDto {
    /// List of providers you want to sync.
    #[serde(rename = "providers", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<Providers>>,
}

impl SyncVoiceLibraryDto {
    pub fn new() -> SyncVoiceLibraryDto {
        SyncVoiceLibraryDto {
            providers: None,
        }
    }
}
/// List of providers you want to sync.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Providers {
    #[serde(rename = "vapi")]
    Vapi,
    #[serde(rename = "11labs")]
    Variant11labs,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "cartesia")]
    Cartesia,
    #[serde(rename = "custom-voice")]
    CustomVoice,
    #[serde(rename = "deepgram")]
    Deepgram,
    #[serde(rename = "hume")]
    Hume,
    #[serde(rename = "lmnt")]
    Lmnt,
    #[serde(rename = "neuphonic")]
    Neuphonic,
    #[serde(rename = "openai")]
    Openai,
    #[serde(rename = "playht")]
    Playht,
    #[serde(rename = "rime-ai")]
    RimeAi,
    #[serde(rename = "smallest-ai")]
    SmallestAi,
    #[serde(rename = "tavus")]
    Tavus,
    #[serde(rename = "sesame")]
    Sesame,
}

impl Default for Providers {
    fn default() -> Providers {
        Self::Vapi
    }
}

