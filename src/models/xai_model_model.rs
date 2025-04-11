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

/// XaiModelModel : This is the name of the model. Ex. cognitivecomputations/dolphin-mixtral-8x7b
/// This is the name of the model. Ex. cognitivecomputations/dolphin-mixtral-8x7b
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum XaiModelModel {
    #[serde(rename = "grok-beta")]
    GrokBeta,
    #[serde(rename = "grok-2")]
    Grok2,
    #[serde(rename = "grok-3")]
    Grok3,

}

impl std::fmt::Display for XaiModelModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GrokBeta => write!(f, "grok-beta"),
            Self::Grok2 => write!(f, "grok-2"),
            Self::Grok3 => write!(f, "grok-3"),
        }
    }
}

impl Default for XaiModelModel {
    fn default() -> XaiModelModel {
        Self::GrokBeta
    }
}

