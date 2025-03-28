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
pub struct ExactReplacement {
    /// This is the exact replacement type. You can use this to replace a specific word or phrase with a different word or phrase.  Usage: - Replace \"hello\" with \"hi\": { type: 'exact', key: 'hello', value: 'hi' } - Replace \"good morning\" with \"good day\": { type: 'exact', key: 'good morning', value: 'good day' } - Replace a specific name: { type: 'exact', key: 'John Doe', value: 'Jane Smith' } - Replace an acronym: { type: 'exact', key: 'AI', value: 'Artificial Intelligence' } - Replace a company name with its phonetic pronunciation: { type: 'exact', key: 'Vapi', value: 'Vappy' }
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the key to replace.
    #[serde(rename = "key")]
    pub key: String,
    /// This is the value that will replace the match.
    #[serde(rename = "value")]
    pub value: String,
}

impl ExactReplacement {
    pub fn new(r#type: Type, key: String, value: String) -> ExactReplacement {
        ExactReplacement { r#type, key, value }
    }
}
/// This is the exact replacement type. You can use this to replace a specific word or phrase with a different word or phrase.  Usage: - Replace \"hello\" with \"hi\": { type: 'exact', key: 'hello', value: 'hi' } - Replace \"good morning\" with \"good day\": { type: 'exact', key: 'good morning', value: 'good day' } - Replace a specific name: { type: 'exact', key: 'John Doe', value: 'Jane Smith' } - Replace an acronym: { type: 'exact', key: 'AI', value: 'Artificial Intelligence' } - Replace a company name with its phonetic pronunciation: { type: 'exact', key: 'Vapi', value: 'Vappy' }
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "exact")]
    Exact,
}

impl Default for Type {
    fn default() -> Type {
        Self::Exact
    }
}
