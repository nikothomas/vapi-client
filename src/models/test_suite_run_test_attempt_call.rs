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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSuiteRunTestAttemptCall {
    /// This is the artifact associated with the call.
    #[serde(rename = "artifact")]
    pub artifact: models::Artifact,
}

impl TestSuiteRunTestAttemptCall {
    pub fn new(artifact: models::Artifact) -> TestSuiteRunTestAttemptCall {
        TestSuiteRunTestAttemptCall { artifact }
    }
}
