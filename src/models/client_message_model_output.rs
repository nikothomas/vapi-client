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
pub struct ClientMessageModelOutput {
    /// This is the type of the message. \"model-output\" is sent as the model outputs tokens.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the output of the model. It can be a token or tool call.
    #[serde(rename = "output")]
    pub output: std::collections::HashMap<String, serde_json::Value>,
}

impl ClientMessageModelOutput {
    pub fn new(r#type: Type, output: std::collections::HashMap<String, serde_json::Value>) -> ClientMessageModelOutput {
        ClientMessageModelOutput {
            r#type,
            output,
        }
    }
}
/// This is the type of the message. \"model-output\" is sent as the model outputs tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "model-output")]
    ModelOutput,
}

impl Default for Type {
    fn default() -> Type {
        Self::ModelOutput
    }
}

