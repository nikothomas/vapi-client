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
pub struct OpenAiFunction {
    /// This is a boolean that controls whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the parameters field. Only a subset of JSON Schema is supported when strict is true. Learn more about Structured Outputs in the [OpenAI guide](https://openai.com/index/introducing-structured-outputs-in-the-api/).  @default false
    #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// This is the the name of the function to be called.  Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    #[serde(rename = "name")]
    pub name: String,
    /// This is the description of what the function does, used by the AI to choose when and how to call the function.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// These are the parameters the functions accepts, described as a JSON Schema object.  See the [OpenAI guide](https://platform.openai.com/docs/guides/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema) for documentation about the format.  Omitting parameters defines a function with an empty parameter list.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<models::OpenAiFunctionParameters>,
}

impl OpenAiFunction {
    pub fn new(name: String) -> OpenAiFunction {
        OpenAiFunction {
            strict: None,
            name,
            description: None,
            parameters: None,
        }
    }
}
