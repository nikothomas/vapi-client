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
pub struct LangfuseObservabilityPlan {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is an array of tags to be added to the Langfuse trace. Tags allow you to categorize and filter traces. https://langfuse.com/docs/tracing-features/tags
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// This is a JSON object that will be added to the Langfuse trace. Traces can be enriched with metadata to better understand your users, application, and experiments. https://langfuse.com/docs/tracing-features/metadata By default it includes the call metadata, assistant metadata, and assistant overrides.
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
}

impl LangfuseObservabilityPlan {
    pub fn new(provider: Provider, tags: Vec<String>) -> LangfuseObservabilityPlan {
        LangfuseObservabilityPlan {
            provider,
            tags,
            metadata: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "langfuse")]
    Langfuse,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Langfuse
    }
}

