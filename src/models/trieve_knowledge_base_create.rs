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
pub struct TrieveKnowledgeBaseCreate {
    /// This is to create a new dataset on Trieve.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// These are the chunk plans used to create the dataset.
    #[serde(rename = "chunkPlans")]
    pub chunk_plans: Vec<models::TrieveKnowledgeBaseChunkPlan>,
}

impl TrieveKnowledgeBaseCreate {
    pub fn new(
        r#type: Type,
        chunk_plans: Vec<models::TrieveKnowledgeBaseChunkPlan>,
    ) -> TrieveKnowledgeBaseCreate {
        TrieveKnowledgeBaseCreate {
            r#type,
            chunk_plans,
        }
    }
}
/// This is to create a new dataset on Trieve.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "create")]
    Create,
}

impl Default for Type {
    fn default() -> Type {
        Self::Create
    }
}
