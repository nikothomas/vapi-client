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
pub struct KnowledgeBaseResponseDocument {
    /// This is the content of the document.
    #[serde(rename = "content")]
    pub content: String,
    /// This is the similarity score of the document.
    #[serde(rename = "similarity")]
    pub similarity: f64,
    /// This is the uuid of the document.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl KnowledgeBaseResponseDocument {
    pub fn new(content: String, similarity: f64) -> KnowledgeBaseResponseDocument {
        KnowledgeBaseResponseDocument {
            content,
            similarity,
            uuid: None,
        }
    }
}
