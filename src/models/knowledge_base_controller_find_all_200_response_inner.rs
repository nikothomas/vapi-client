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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(tag = "provider")]
pub enum KnowledgeBaseControllerFindAll200ResponseInner {
    #[serde(rename = "trieve")]
    Trieve(models::TrieveKnowledgeBase),
    #[serde(rename = "custom-knowledge-base")]
    CustomKnowledgeBase(models::CustomKnowledgeBase),
}

impl Default for KnowledgeBaseControllerFindAll200ResponseInner {
    fn default() -> Self {
        Self::Trieve(Default::default())
    }
}
