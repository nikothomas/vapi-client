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
pub struct CreateTestSuiteTestChatDto {
    /// These are the scorers used to evaluate the test.
    #[serde(rename = "scorers")]
    pub scorers: Vec<models::TestSuiteTestVoiceScorersInner>,
    /// This is the type of the test, which must be chat.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the script to be used for the chat test.
    #[serde(rename = "script")]
    pub script: String,
    /// This is the number of attempts allowed for the test.
    #[serde(rename = "numAttempts", skip_serializing_if = "Option::is_none")]
    pub num_attempts: Option<f64>,
    /// This is the name of the test.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateTestSuiteTestChatDto {
    pub fn new(scorers: Vec<models::TestSuiteTestVoiceScorersInner>, r#type: Type, script: String) -> CreateTestSuiteTestChatDto {
        CreateTestSuiteTestChatDto {
            scorers,
            r#type,
            script,
            num_attempts: None,
            name: None,
        }
    }
}
/// This is the type of the test, which must be chat.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "chat")]
    Chat,
}

impl Default for Type {
    fn default() -> Type {
        Self::Chat
    }
}

