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
pub struct BothCustomEndpointingRule {
    /// This endpointing rule is based on both the last assistant message and the current customer message as they are speaking.  Flow: - Assistant speaks - Customer starts speaking - Customer transcription comes in - This rule is evaluated on the last assistant message and the current customer transcription - If assistant message matches `assistantRegex` AND customer message matches `customerRegex`, the endpointing timeout is set to `timeoutSeconds`  Usage: - If you want to wait longer while customer is speaking numbers, you can set a longer timeout.
    #[serde(rename = "type")]
    pub r#type: TypeTrue,
    /// This is the regex pattern to match the assistant's message.  Note: - This works by using the `RegExp.test` method in Node.JS. Eg. `/hello/.test(\"hello there\")` will return `true`.  Hot tip: - In JavaScript, escape `\\` when sending the regex pattern. Eg. `\"hello\\sthere\"` will be sent over the wire as `\"hellosthere\"`. Send `\"hello\\\\sthere\"` instead. - `RegExp.test` does substring matching, so `/cat/.test(\"I love cats\")` will return `true`. To do full string matching, send \"^cat$\".
    #[serde(rename = "assistantRegex")]
    pub assistant_regex: String,
    /// These are the options for the assistant's message regex match. Defaults to all disabled.  @default []
    #[serde(
        rename = "assistantRegexOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub assistant_regex_options: Option<Vec<models::RegexOption>>,
    #[serde(rename = "customerRegex")]
    pub customer_regex: String,
    /// These are the options for the customer's message regex match. Defaults to all disabled.  @default []
    #[serde(
        rename = "customerRegexOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_regex_options: Option<Vec<models::RegexOption>>,
    /// This is the endpointing timeout in seconds, if the rule is matched.
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: f64,
}

impl BothCustomEndpointingRule {
    pub fn new(
        r#type: TypeTrue,
        assistant_regex: String,
        customer_regex: String,
        timeout_seconds: f64,
    ) -> BothCustomEndpointingRule {
        BothCustomEndpointingRule {
            r#type,
            assistant_regex,
            assistant_regex_options: None,
            customer_regex,
            customer_regex_options: None,
            timeout_seconds,
        }
    }
}
/// This endpointing rule is based on both the last assistant message and the current customer message as they are speaking.  Flow: - Assistant speaks - Customer starts speaking - Customer transcription comes in - This rule is evaluated on the last assistant message and the current customer transcription - If assistant message matches `assistantRegex` AND customer message matches `customerRegex`, the endpointing timeout is set to `timeoutSeconds`  Usage: - If you want to wait longer while customer is speaking numbers, you can set a longer timeout.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "both")]
    Both,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Both
    }
}
