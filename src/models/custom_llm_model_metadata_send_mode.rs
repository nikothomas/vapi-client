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

/// CustomLlmModelMetadataSendMode : This determines whether metadata is sent in requests to the custom provider.  - `off` will not send any metadata. payload will look like `{ messages }` - `variable` will send `assistant.metadata` as a variable on the payload. payload will look like `{ messages, metadata }` - `destructured` will send `assistant.metadata` fields directly on the payload. payload will look like `{ messages, ...metadata }`  Further, `variable` and `destructured` will send `call`, `phoneNumber`, and `customer` objects in the payload.  Default is `variable`.
/// This determines whether metadata is sent in requests to the custom provider.  - `off` will not send any metadata. payload will look like `{ messages }` - `variable` will send `assistant.metadata` as a variable on the payload. payload will look like `{ messages, metadata }` - `destructured` will send `assistant.metadata` fields directly on the payload. payload will look like `{ messages, ...metadata }`  Further, `variable` and `destructured` will send `call`, `phoneNumber`, and `customer` objects in the payload.  Default is `variable`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CustomLlmModelMetadataSendMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "variable")]
    Variable,
    #[serde(rename = "destructured")]
    Destructured,

}

impl std::fmt::Display for CustomLlmModelMetadataSendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::Variable => write!(f, "variable"),
            Self::Destructured => write!(f, "destructured"),
        }
    }
}

impl Default for CustomLlmModelMetadataSendMode {
    fn default() -> CustomLlmModelMetadataSendMode {
        Self::Off
    }
}

