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
pub struct ClientMessageModelOutput {
    /// This is the type of the message. \"model-output\" is sent as the model outputs tokens.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the output of the model. It can be a token or tool call.
    #[serde(rename = "output")]
    pub output: serde_json::Value,
}

impl ClientMessageModelOutput {
    pub fn new(r#type: Type, output: serde_json::Value) -> ClientMessageModelOutput {
        ClientMessageModelOutput { r#type, output }
    }
}
/// This is the type of the message. \"model-output\" is sent as the model outputs tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "model-output")]
    ModelOutput,
}

impl Default for Type {
    fn default() -> Type {
        Self::ModelOutput
    }
}
