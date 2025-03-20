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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GhlToolProviderDetails {
    /// This is the Template URL or the Snapshot URL corresponding to the Template.
    #[serde(rename = "templateUrl", skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
    #[serde(rename = "setupInstructions", skip_serializing_if = "Option::is_none")]
    pub setup_instructions: Option<Vec<models::ToolTemplateSetup>>,
    /// The type of tool. \"ghl\" for GHL tool.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowName", skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    #[serde(rename = "webhookHookId", skip_serializing_if = "Option::is_none")]
    pub webhook_hook_id: Option<String>,
    #[serde(rename = "webhookHookName", skip_serializing_if = "Option::is_none")]
    pub webhook_hook_name: Option<String>,
    #[serde(rename = "locationId", skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
}

impl GhlToolProviderDetails {
    pub fn new(r#type: Type) -> GhlToolProviderDetails {
        GhlToolProviderDetails {
            template_url: None,
            setup_instructions: None,
            r#type,
            workflow_id: None,
            workflow_name: None,
            webhook_hook_id: None,
            webhook_hook_name: None,
            location_id: None,
        }
    }
}
/// The type of tool. \"ghl\" for GHL tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ghl")]
    Ghl,
}

impl Default for Type {
    fn default() -> Type {
        Self::Ghl
    }
}
