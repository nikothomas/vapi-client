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
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct MakeToolMetadata {
    #[serde(rename = "scenarioId", skip_serializing_if = "Option::is_none")]
    pub scenario_id: Option<f64>,
    #[serde(rename = "triggerHookId", skip_serializing_if = "Option::is_none")]
    pub trigger_hook_id: Option<f64>,
}

impl MakeToolMetadata {
    pub fn new() -> MakeToolMetadata {
        MakeToolMetadata {
            scenario_id: None,
            trigger_hook_id: None,
        }
    }
}
