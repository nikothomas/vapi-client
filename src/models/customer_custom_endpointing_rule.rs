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
pub struct CustomerCustomEndpointingRule {
    /// This endpointing rule is based on current customer message as they are speaking.  Flow: - Assistant speaks - Customer starts speaking - Customer transcription comes in - This rule is evaluated on the current customer transcription - If a match is found based on `regex`, the endpointing timeout is set to `timeoutSeconds`  Usage: - If you want to wait longer while customer is speaking numbers, you can set a longer timeout.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the regex pattern to match.  Note: - This works by using the `RegExp.test` method in Node.JS. Eg. `/hello/.test(\"hello there\")` will return `true`.  Hot tip: - In JavaScript, escape `\\` when sending the regex pattern. Eg. `\"hello\\sthere\"` will be sent over the wire as `\"hellosthere\"`. Send `\"hello\\\\sthere\"` instead. - `RegExp.test` does substring matching, so `/cat/.test(\"I love cats\")` will return `true`. To do full string matching, send \"^cat$\".
    #[serde(rename = "regex")]
    pub regex: String,
    /// These are the options for the regex match. Defaults to all disabled.  @default []
    #[serde(rename = "regexOptions", skip_serializing_if = "Option::is_none")]
    pub regex_options: Option<Vec<models::RegexOption>>,
    /// This is the endpointing timeout in seconds, if the rule is matched.
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: f64,
}

impl CustomerCustomEndpointingRule {
    pub fn new(r#type: Type, regex: String, timeout_seconds: f64) -> CustomerCustomEndpointingRule {
        CustomerCustomEndpointingRule {
            r#type,
            regex,
            regex_options: None,
            timeout_seconds,
        }
    }
}
/// This endpointing rule is based on current customer message as they are speaking.  Flow: - Assistant speaks - Customer starts speaking - Customer transcription comes in - This rule is evaluated on the current customer transcription - If a match is found based on `regex`, the endpointing timeout is set to `timeoutSeconds`  Usage: - If you want to wait longer while customer is speaking numbers, you can set a longer timeout.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "customer")]
    Customer,
}

impl Default for Type {
    fn default() -> Type {
        Self::Customer
    }
}

