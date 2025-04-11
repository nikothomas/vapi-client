/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TestSuiteRunScorerAiResult : This is the result of the test suite.
/// This is the result of the test suite.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TestSuiteRunScorerAiResult {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,

}

impl std::fmt::Display for TestSuiteRunScorerAiResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pass => write!(f, "pass"),
            Self::Fail => write!(f, "fail"),
        }
    }
}

impl Default for TestSuiteRunScorerAiResult {
    fn default() -> TestSuiteRunScorerAiResult {
        Self::Pass
    }
}

