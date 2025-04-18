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

/// TestSuiteRunTestResultTest : This is the test that was run.
/// This is the test that was run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSuiteRunTestResultTest {
    TestSuiteTestVoice(models::TestSuiteTestVoice),
}

impl Default for TestSuiteRunTestResultTest {
    fn default() -> Self {
        Self::TestSuiteTestVoice(Default::default())
    }
}
/// This is the type of the test, which must be voice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "voice")]
    Voice,
}

impl Default for Type {
    fn default() -> Type {
        Self::Voice
    }
}

