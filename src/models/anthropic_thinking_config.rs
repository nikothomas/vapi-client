/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnthropicThinkingConfig {
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The maximum number of tokens to allocate for thinking. Must be between 1024 and 100000 tokens.
    #[serde(rename = "budgetTokens")]
    pub budget_tokens: f64,
}

impl AnthropicThinkingConfig {
    pub fn new(r#type: Type, budget_tokens: f64) -> AnthropicThinkingConfig {
        AnthropicThinkingConfig {
            r#type,
            budget_tokens,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "enabled")]
    Enabled,
}

impl Default for Type {
    fn default() -> Type {
        Self::Enabled
    }
}

