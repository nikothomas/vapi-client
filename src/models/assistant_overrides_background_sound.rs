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

/// AssistantOverridesBackgroundSound : This is the background sound in the call. Default for phone calls is 'office' and default for web calls is 'off'. You can also provide a custom sound by providing a URL to an audio file.
/// This is the background sound in the call. Default for phone calls is 'office' and default for web calls is 'off'. You can also provide a custom sound by providing a URL to an audio file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantOverridesBackgroundSound {
    AssistantOverridesBackgroundSoundZero(models::AssistantOverridesBackgroundSoundZero),
    String(String),
}

impl Default for AssistantOverridesBackgroundSound {
    fn default() -> Self {
        Self::AssistantOverridesBackgroundSoundZero(Default::default())
    }
}

