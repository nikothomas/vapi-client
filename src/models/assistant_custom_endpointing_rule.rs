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
pub struct AssistantCustomEndpointingRule {
    /// This endpointing rule is based on the last assistant message before customer started speaking.  Flow: - Assistant speaks - Customer starts speaking - Customer transcription comes in - This rule is evaluated on the last assistant message - If a match is found based on `regex`, the endpointing timeout is set to `timeoutSeconds`  Usage: - If you have yes/no questions in your use case like \"are you interested in a loan?\", you can set a shorter timeout. - If you have questions where the customer may pause to look up information like \"what's my account number?\", you can set a longer timeout.
    #[serde(rename = "type")]
    pub r#type: TypeTrue,
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

impl AssistantCustomEndpointingRule {
    pub fn new(
        r#type: TypeTrue,
        regex: String,
        timeout_seconds: f64,
    ) -> AssistantCustomEndpointingRule {
        AssistantCustomEndpointingRule {
            r#type,
            regex,
            regex_options: None,
            timeout_seconds,
        }
    }
}
/// This endpointing rule is based on the last assistant message before customer started speaking.  Flow: - Assistant speaks - Customer starts speaking - Customer transcription comes in - This rule is evaluated on the last assistant message - If a match is found based on `regex`, the endpointing timeout is set to `timeoutSeconds`  Usage: - If you have yes/no questions in your use case like \"are you interested in a loan?\", you can set a shorter timeout. - If you have questions where the customer may pause to look up information like \"what's my account number?\", you can set a longer timeout.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "assistant")]
    Assistant,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Assistant
    }
}
