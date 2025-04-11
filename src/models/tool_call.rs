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
pub struct ToolCall {
    /// This is the type of tool the model called.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "function")]
    pub function: models::ToolCallFunction,
    /// This is the unique identifier for the tool call.
    #[serde(rename = "id")]
    pub id: String,
}

impl ToolCall {
    pub fn new(r#type: Type, function: models::ToolCallFunction, id: String) -> ToolCall {
        ToolCall {
            r#type,
            function,
            id,
        }
    }
}
/// This is the type of tool the model called.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "function")]
    Function,
}

impl Default for Type {
    fn default() -> Type {
        Self::Function
    }
}

