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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSuite {
    /// This is the unique identifier for the test suite.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the org that this test suite belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the test suite was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the test suite was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the name of the test suite.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// This is the phone number ID associated with this test suite.
    #[serde(rename = "phoneNumberId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<Option<String>>,
    #[serde(rename = "testerPlan", skip_serializing_if = "Option::is_none")]
    pub tester_plan: Option<models::TesterPlan>,
    #[serde(rename = "targetPlan", skip_serializing_if = "Option::is_none")]
    pub target_plan: Option<models::TargetPlan>,
}

impl TestSuite {
    pub fn new(id: String, org_id: String, created_at: String, updated_at: String) -> TestSuite {
        TestSuite {
            id,
            org_id,
            created_at,
            updated_at,
            name: None,
            phone_number_id: None,
            tester_plan: None,
            target_plan: None,
        }
    }
}

