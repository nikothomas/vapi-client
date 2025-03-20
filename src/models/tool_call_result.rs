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
pub struct ToolCallResult {
    /// This is the message that will be spoken to the user.  If this is not returned, assistant will speak: 1. a `request-complete` or `request-failed` message from `tool.messages`, if it exists 2. a response generated by the model, if not
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Vec<models::ToolCallResultMessageInner>>,
    /// This is the name of the function the model called.
    #[serde(rename = "name")]
    pub name: String,
    /// This is the unique identifier for the tool call.
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,
    /// This is the result if the tool call was successful. This is added to the conversation history.  Further, if this is returned, assistant will speak: 1. the `message`, if it exists and is of type `request-complete` 2. a `request-complete` message from `tool.messages`, if it exists 3. a response generated by the model, if neither exist
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// This is the error if the tool call was not successful. This is added to the conversation history.  Further, if this is returned, assistant will speak: 1. the `message`, if it exists and is of type `request-failed` 2. a `request-failed` message from `tool.messages`, if it exists 3. a response generated by the model, if neither exist
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ToolCallResult {
    pub fn new(name: String, tool_call_id: String) -> ToolCallResult {
        ToolCallResult {
            message: None,
            name,
            tool_call_id,
            result: None,
            error: None,
        }
    }
}
