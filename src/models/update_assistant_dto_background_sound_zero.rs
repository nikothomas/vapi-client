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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UpdateAssistantDtoBackgroundSoundZero {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "office")]
    Office,

}

impl std::fmt::Display for UpdateAssistantDtoBackgroundSoundZero {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::Office => write!(f, "office"),
        }
    }
}

impl Default for UpdateAssistantDtoBackgroundSoundZero {
    fn default() -> UpdateAssistantDtoBackgroundSoundZero {
        Self::Off
    }
}

