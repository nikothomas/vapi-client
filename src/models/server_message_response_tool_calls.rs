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
pub struct ServerMessageResponseToolCalls {
    /// These are the results of the \"tool-calls\" message.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::ToolCallResult>>,
    /// This is the error message if the tool call was not successful.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ServerMessageResponseToolCalls {
    pub fn new() -> ServerMessageResponseToolCalls {
        ServerMessageResponseToolCalls {
            results: None,
            error: None,
        }
    }
}

