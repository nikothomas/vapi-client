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
