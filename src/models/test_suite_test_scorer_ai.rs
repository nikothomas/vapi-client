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
pub struct TestSuiteTestScorerAi {
    /// This is the type of the scorer, which must be AI.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the rubric used by the AI scorer.
    #[serde(rename = "rubric")]
    pub rubric: String,
}

impl TestSuiteTestScorerAi {
    pub fn new(r#type: Type, rubric: String) -> TestSuiteTestScorerAi {
        TestSuiteTestScorerAi { r#type, rubric }
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
