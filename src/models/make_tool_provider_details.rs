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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MakeToolProviderDetails {
    /// This is the Template URL or the Snapshot URL corresponding to the Template.
    #[serde(rename = "templateUrl", skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
    #[serde(rename = "setupInstructions", skip_serializing_if = "Option::is_none")]
    pub setup_instructions: Option<Vec<models::ToolTemplateSetup>>,
    /// The type of tool. \"make\" for Make tool.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "scenarioId", skip_serializing_if = "Option::is_none")]
    pub scenario_id: Option<f64>,
    #[serde(rename = "scenarioName", skip_serializing_if = "Option::is_none")]
    pub scenario_name: Option<String>,
    #[serde(rename = "triggerHookId", skip_serializing_if = "Option::is_none")]
    pub trigger_hook_id: Option<f64>,
    #[serde(rename = "triggerHookName", skip_serializing_if = "Option::is_none")]
    pub trigger_hook_name: Option<String>,
}

impl MakeToolProviderDetails {
    pub fn new(r#type: Type) -> MakeToolProviderDetails {
        MakeToolProviderDetails {
            template_url: None,
            setup_instructions: None,
            r#type,
            scenario_id: None,
            scenario_name: None,
            trigger_hook_id: None,
            trigger_hook_name: None,
        }
    }
}
/// The type of tool. \"make\" for Make tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "make")]
    Make,
}

impl Default for Type {
    fn default() -> Type {
        Self::Make
    }
}

