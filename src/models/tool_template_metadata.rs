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
pub struct ToolTemplateMetadata {
    #[serde(rename = "collectionType", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "collectionName", skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
}

impl ToolTemplateMetadata {
    pub fn new() -> ToolTemplateMetadata {
        ToolTemplateMetadata {
            collection_type: None,
            collection_id: None,
            collection_name: None,
        }
    }
}
