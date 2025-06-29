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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartSpeakingPlanSmartEndpointingEnabled {
    Boolean(bool),
    String(String),
}

impl Default for StartSpeakingPlanSmartEndpointingEnabled {
    fn default() -> Self {
        Self::Boolean(Default::default())
    }
}
