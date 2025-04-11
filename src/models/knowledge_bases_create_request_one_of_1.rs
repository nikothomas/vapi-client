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
pub struct KnowledgeBasesCreateRequestOneOf1 {
    #[serde(rename = "server")]
    pub server: Box<models::Server>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl KnowledgeBasesCreateRequestOneOf1 {
    pub fn new(server: models::Server, provider: Provider) -> KnowledgeBasesCreateRequestOneOf1 {
        KnowledgeBasesCreateRequestOneOf1 {
            server: Box::new(server),
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "custom-knowledge-base")]
    CustomKnowledgeBase,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::CustomKnowledgeBase
    }
}

