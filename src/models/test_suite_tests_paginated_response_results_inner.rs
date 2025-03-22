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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum TestSuiteTestsPaginatedResponseResultsInner {
    TestSuiteTestVoice(models::TestSuiteTestVoice),
}

impl Default for TestSuiteTestsPaginatedResponseResultsInner {
    fn default() -> Self {
        Self::TestSuiteTestVoice(Default::default())
    }
}
/// This is the type of the test, which must be voice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Type {
    #[serde(rename = "voice")]
    Voice,
}

impl Default for Type {
    fn default() -> Type {
        Self::Voice
    }
}
