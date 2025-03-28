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
#[serde(untagged)]
pub enum TestSuiteRunTestAttemptScorerResultsInner {
    TestSuiteRunScorerAi(models::TestSuiteRunScorerAi),
}

impl Default for TestSuiteRunTestAttemptScorerResultsInner {
    fn default() -> Self {
        Self::TestSuiteRunScorerAi(Default::default())
    }
}
/// This is the type of the scorer, which must be AI.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "ai")]
    Ai,
}

impl Default for Type {
    fn default() -> Type {
        Self::Ai
    }
}
/// This is the result of the test suite.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Result {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
}

impl Default for Result {
    fn default() -> Result {
        Self::Pass
    }
}
