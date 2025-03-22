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
pub struct UpdateTestSuiteDto {
    /// This is the name of the test suite.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the phone number ID associated with this test suite.
    #[serde(rename = "phoneNumberId", skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
}

impl UpdateTestSuiteDto {
    pub fn new() -> UpdateTestSuiteDto {
        UpdateTestSuiteDto {
            name: None,
            phone_number_id: None,
        }
    }
}
