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
pub struct TestSuiteRunTestAttempt {
    /// These are the results of the scorers used to evaluate the test attempt.
    #[serde(rename = "scorerResults")]
    pub scorer_results: Vec<models::TestSuiteRunTestAttemptScorerResultsInner>,
    /// This is the call made during the test attempt.
    #[serde(rename = "call")]
    pub call: Box<models::TestSuiteRunTestAttemptCall>,
}

impl TestSuiteRunTestAttempt {
    pub fn new(scorer_results: Vec<models::TestSuiteRunTestAttemptScorerResultsInner>, call: models::TestSuiteRunTestAttemptCall) -> TestSuiteRunTestAttempt {
        TestSuiteRunTestAttempt {
            scorer_results,
            call: Box::new(call),
        }
    }
}

