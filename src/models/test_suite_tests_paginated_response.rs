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
pub struct TestSuiteTestsPaginatedResponse {
    /// A list of test suite tests.
    #[serde(rename = "results")]
    pub results: Vec<models::TestSuiteTestsPaginatedResponseResultsInner>,
    /// Metadata about the pagination.
    #[serde(rename = "metadata")]
    pub metadata: models::PaginationMeta,
}

impl TestSuiteTestsPaginatedResponse {
    pub fn new(
        results: Vec<models::TestSuiteTestsPaginatedResponseResultsInner>,
        metadata: models::PaginationMeta,
    ) -> TestSuiteTestsPaginatedResponse {
        TestSuiteTestsPaginatedResponse { results, metadata }
    }
}
