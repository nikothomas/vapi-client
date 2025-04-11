/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSuiteRunScorerAi {
    /// This is the type of the scorer, which must be AI.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "result")]
    pub result: models::TestSuiteRunScorerAiResult,
    /// This is the reasoning provided by the AI scorer.
    #[serde(rename = "reasoning")]
    pub reasoning: String,
    /// This is the rubric used by the AI scorer.
    #[serde(rename = "rubric")]
    pub rubric: String,
}

impl TestSuiteRunScorerAi {
    pub fn new(r#type: Type, result: models::TestSuiteRunScorerAiResult, reasoning: String, rubric: String) -> TestSuiteRunScorerAi {
        TestSuiteRunScorerAi {
            r#type,
            result,
            reasoning,
            rubric,
        }
    }
}
/// This is the type of the scorer, which must be AI.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ai")]
    Ai,
}

impl Default for Type {
    fn default() -> Type {
        Self::Ai
    }
}

