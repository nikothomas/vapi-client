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
pub struct ServerMessageResponseKnowledgeBaseRequest {
    /// This is the list of documents that will be sent to the model alongside the `messages` to generate a response.
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<models::KnowledgeBaseResponseDocument>>,
    /// This can be used to skip the model output generation and speak a custom message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<models::CustomMessage>,
}

impl ServerMessageResponseKnowledgeBaseRequest {
    pub fn new() -> ServerMessageResponseKnowledgeBaseRequest {
        ServerMessageResponseKnowledgeBaseRequest {
            documents: None,
            message: None,
        }
    }
}
