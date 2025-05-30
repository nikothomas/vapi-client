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
pub struct ChatCompletionMessageWorkflows {
    #[serde(rename = "role")]
    pub role: serde_json::Value,
    #[serde(rename = "content", deserialize_with = "Option::deserialize")]
    pub content: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<models::ChatCompletionMessageMetadata>,
}

impl ChatCompletionMessageWorkflows {
    pub fn new(role: serde_json::Value, content: Option<String>) -> ChatCompletionMessageWorkflows {
        ChatCompletionMessageWorkflows {
            role,
            content,
            metadata: None,
        }
    }
}

