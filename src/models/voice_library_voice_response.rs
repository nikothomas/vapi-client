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
pub struct VoiceLibraryVoiceResponse {
    #[serde(rename = "voiceId")]
    pub voice_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "publicOwnerId", skip_serializing_if = "Option::is_none")]
    pub public_owner_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "age", skip_serializing_if = "Option::is_none")]
    pub age: Option<serde_json::Value>,
    #[serde(rename = "accent", skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
}

impl VoiceLibraryVoiceResponse {
    pub fn new(voice_id: String, name: String) -> VoiceLibraryVoiceResponse {
        VoiceLibraryVoiceResponse {
            voice_id,
            name,
            public_owner_id: None,
            description: None,
            gender: None,
            age: None,
            accent: None,
        }
    }
}
