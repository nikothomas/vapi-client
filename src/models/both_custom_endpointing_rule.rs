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
pub struct BothCustomEndpointingRule {
    /// This is the regex pattern to match the assistant's message.  Note: - This works by using the `RegExp.test` method in Node.JS. Eg. `/hello/.test(\"hello there\")` will return `true`.  Hot tip: - In JavaScript, escape `\\` when sending the regex pattern. Eg. `\"hello\\sthere\"` will be sent over the wire as `\"hellosthere\"`. Send `\"hello\\\\sthere\"` instead. - `RegExp.test` does substring matching, so `/cat/.test(\"I love cats\")` will return `true`. To do full string matching, send \"^cat$\".
    #[serde(rename = "assistantRegex")]
    pub assistant_regex: String,
    /// These are the options for the assistant's message regex match. Defaults to all disabled.  @default []
    #[serde(rename = "assistantRegexOptions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assistant_regex_options: Option<Option<Vec<models::RegexOption>>>,
    #[serde(rename = "customerRegex")]
    pub customer_regex: String,
    /// These are the options for the customer's message regex match. Defaults to all disabled.  @default []
    #[serde(rename = "customerRegexOptions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub customer_regex_options: Option<Option<Vec<models::RegexOption>>>,
    /// This is the endpointing timeout in seconds, if the rule is matched.
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: f64,
}

impl BothCustomEndpointingRule {
    pub fn new(assistant_regex: String, customer_regex: String, timeout_seconds: f64) -> BothCustomEndpointingRule {
        BothCustomEndpointingRule {
            assistant_regex,
            assistant_regex_options: None,
            customer_regex,
            customer_regex_options: None,
            timeout_seconds,
        }
    }
}

