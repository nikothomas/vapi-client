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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TestSuitesPaginatedResponse {
    #[serde(rename = "results")]
    pub results: Vec<models::TestSuite>,
    #[serde(rename = "metadata")]
    pub metadata: models::PaginationMeta,
}

impl TestSuitesPaginatedResponse {
    pub fn new(
        results: Vec<models::TestSuite>,
        metadata: models::PaginationMeta,
    ) -> TestSuitesPaginatedResponse {
        TestSuitesPaginatedResponse { results, metadata }
    }
}
