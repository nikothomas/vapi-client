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
pub struct MakeToolMetadata {
    #[serde(rename = "scenarioId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scenario_id: Option<Option<f64>>,
    #[serde(rename = "triggerHookId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trigger_hook_id: Option<Option<f64>>,
}

impl MakeToolMetadata {
    pub fn new() -> MakeToolMetadata {
        MakeToolMetadata {
            scenario_id: None,
            trigger_hook_id: None,
        }
    }
}

