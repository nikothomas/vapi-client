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
pub struct FormatPlanReplacementsItemOneOf {
    /// This is the key to replace.
    #[serde(rename = "key")]
    pub key: String,
    /// This is the value that will replace the match.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl FormatPlanReplacementsItemOneOf {
    pub fn new(key: String, value: String, r#type: Type) -> FormatPlanReplacementsItemOneOf {
        FormatPlanReplacementsItemOneOf {
            key,
            value,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "exact")]
    Exact,
}

impl Default for Type {
    fn default() -> Type {
        Self::Exact
    }
}

