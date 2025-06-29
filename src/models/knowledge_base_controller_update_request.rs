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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub enum KnowledgeBaseControllerUpdateRequest {
    #[serde(rename = "trieve")]
    Trieve(models::UpdateTrieveKnowledgeBaseDto),
    #[serde(rename = "custom-knowledge-base")]
    CustomKnowledgeBase(models::UpdateCustomKnowledgeBaseDto),
}

impl Default for KnowledgeBaseControllerUpdateRequest {
    fn default() -> Self {
        Self::Trieve(Default::default())
    }
}
