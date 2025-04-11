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
pub struct TestSuitePhoneNumber {
    /// This is the provider of the phone number.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the phone number that is being tested.
    #[serde(rename = "number")]
    pub number: String,
}

impl TestSuitePhoneNumber {
    pub fn new(provider: Provider, number: String) -> TestSuitePhoneNumber {
        TestSuitePhoneNumber {
            provider,
            number,
        }
    }
}
/// This is the provider of the phone number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "test-suite")]
    TestSuite,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::TestSuite
    }
}

