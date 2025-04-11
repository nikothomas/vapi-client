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
pub struct Transport {
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<models::TransportProvider>,
    /// This is determines whether the assistant will have video enabled.  Only relevant for `webCall` type.
    #[serde(rename = "assistantVideoEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assistant_video_enabled: Option<Option<bool>>,
}

impl Transport {
    pub fn new() -> Transport {
        Transport {
            provider: None,
            assistant_video_enabled: None,
        }
    }
}

