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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ToolCallFunction {
    /// This is the name of the function the model called.
    #[serde(rename = "name")]
    pub name: String,
    /// These are the arguments that the function was called with.
    #[serde(rename = "arguments")]
    pub arguments: serde_json::Value,
}

impl ToolCallFunction {
    pub fn new(name: String, arguments: serde_json::Value) -> ToolCallFunction {
        ToolCallFunction { name, arguments }
    }
}
