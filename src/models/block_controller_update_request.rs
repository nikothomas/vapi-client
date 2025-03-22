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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum BlockControllerUpdateRequest {
    #[serde(rename = "conversation")]
    Conversation(models::UpdateConversationBlockDto),
    #[serde(rename = "tool-call")]
    ToolCall(models::UpdateToolCallBlockDto),
    #[serde(rename = "workflow")]
    Workflow(models::UpdateWorkflowBlockDto),
}

impl Default for BlockControllerUpdateRequest {
    fn default() -> Self {
        Self::Conversation(Default::default())
    }
}
