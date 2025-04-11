/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FallbackTranscriberPlanTranscribersItemOneOf1 {
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::FallbackAzureSpeechTranscriberLanguage>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl FallbackTranscriberPlanTranscribersItemOneOf1 {
    pub fn new(provider: Provider) -> FallbackTranscriberPlanTranscribersItemOneOf1 {
        FallbackTranscriberPlanTranscribersItemOneOf1 {
            language: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "azure")]
    Azure,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Azure
    }
}

