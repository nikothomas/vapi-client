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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantPaginatedResponse {
    #[serde(rename = "results")]
    pub results: Vec<models::Assistant>,
    #[serde(rename = "metadata")]
    pub metadata: models::PaginationMeta,
}

impl AssistantPaginatedResponse {
    pub fn new(
        results: Vec<models::Assistant>,
        metadata: models::PaginationMeta,
    ) -> AssistantPaginatedResponse {
        AssistantPaginatedResponse { results, metadata }
    }
}
