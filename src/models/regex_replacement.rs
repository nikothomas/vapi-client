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
pub struct RegexReplacement {
    /// This is the regex replacement type. You can use this to replace a word or phrase that matches a pattern.  Usage: - Replace all numbers with \"some number\": { type: 'regex', regex: '\\\\d+', value: 'some number' } - Replace email addresses with \"[EMAIL]\": { type: 'regex', regex: '\\\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\\\.[A-Z|a-z]{2,}\\\\b', value: '[EMAIL]' } - Replace phone numbers with a formatted version: { type: 'regex', regex: '(\\\\d{3})(\\\\d{3})(\\\\d{4})', value: '($1) $2-$3' } - Replace all instances of \"color\" or \"colour\" with \"hue\": { type: 'regex', regex: 'colou?r', value: 'hue' } - Capitalize the first letter of every sentence: { type: 'regex', regex: '(?<=\\\\. |^)[a-z]', value: (match) => match.toUpperCase() }
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the regex pattern to replace.  Note: - This works by using the `string.replace` method in Node.JS. Eg. `\"hello there\".replace(/hello/g, \"hi\")` will return `\"hi there\"`.  Hot tip: - In JavaScript, escape `\\` when sending the regex pattern. Eg. `\"hello\\sthere\"` will be sent over the wire as `\"hellosthere\"`. Send `\"hello\\\\sthere\"` instead.
    #[serde(rename = "regex")]
    pub regex: String,
    /// These are the options for the regex replacement. Defaults to all disabled.  @default []
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::RegexOption>>,
    /// This is the value that will replace the match.
    #[serde(rename = "value")]
    pub value: String,
}

impl RegexReplacement {
    pub fn new(r#type: Type, regex: String, value: String) -> RegexReplacement {
        RegexReplacement {
            r#type,
            regex,
            options: None,
            value,
        }
    }
}
/// This is the regex replacement type. You can use this to replace a word or phrase that matches a pattern.  Usage: - Replace all numbers with \"some number\": { type: 'regex', regex: '\\\\d+', value: 'some number' } - Replace email addresses with \"[EMAIL]\": { type: 'regex', regex: '\\\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\\\.[A-Z|a-z]{2,}\\\\b', value: '[EMAIL]' } - Replace phone numbers with a formatted version: { type: 'regex', regex: '(\\\\d{3})(\\\\d{3})(\\\\d{4})', value: '($1) $2-$3' } - Replace all instances of \"color\" or \"colour\" with \"hue\": { type: 'regex', regex: 'colou?r', value: 'hue' } - Capitalize the first letter of every sentence: { type: 'regex', regex: '(?<=\\\\. |^)[a-z]', value: (match) => match.toUpperCase() }
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "regex")]
    Regex,
}

impl Default for Type {
    fn default() -> Type {
        Self::Regex
    }
}
