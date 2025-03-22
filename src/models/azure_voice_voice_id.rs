/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


use crate::models;

/// AzureVoiceVoiceId : This is the provider-specific ID that will be used.
/// This is the provider-specific ID that will be used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum AzureVoiceVoiceId {
    PresetVoiceOptions(String),
    AzureVoiceId(String),
}

impl Default for AzureVoiceVoiceId {
    fn default() -> Self {
        Self::PresetVoiceOptions(Default::default())
    }
}
