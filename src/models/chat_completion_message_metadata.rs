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
pub struct ChatCompletionMessageMetadata {
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "taskType")]
    pub task_type: String,
    #[serde(rename = "taskOutput")]
    pub task_output: String,
    #[serde(rename = "taskState", skip_serializing_if = "Option::is_none")]
    pub task_state: Option<serde_json::Value>,
    #[serde(rename = "nodeTrace", skip_serializing_if = "Option::is_none")]
    pub node_trace: Option<Vec<String>>,
}

impl ChatCompletionMessageMetadata {
    pub fn new(
        task_name: String,
        task_type: String,
        task_output: String,
    ) -> ChatCompletionMessageMetadata {
        ChatCompletionMessageMetadata {
            task_name,
            task_type,
            task_output,
            task_state: None,
            node_trace: None,
        }
    }
}
