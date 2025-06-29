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
pub struct GhlToolMetadata {
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(rename = "locationId", skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
}

impl GhlToolMetadata {
    pub fn new() -> GhlToolMetadata {
        GhlToolMetadata {
            workflow_id: None,
            location_id: None,
        }
    }
}
