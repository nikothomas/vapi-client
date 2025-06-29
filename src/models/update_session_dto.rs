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
pub struct UpdateSessionDto {
    /// This is the new name for the session. Maximum length is 40 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the new status for the session.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusTrue>,
    /// Session expiration time in seconds. Defaults to 24 hours (86400 seconds) if not set.
    #[serde(rename = "expirationSeconds", skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<f64>,
    /// This is the updated array of chat messages.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::MessageArrayInner>>,
}

impl UpdateSessionDto {
    pub fn new() -> UpdateSessionDto {
        UpdateSessionDto {
            name: None,
            status: None,
            expiration_seconds: None,
            messages: None,
        }
    }
}
/// This is the new status for the session.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusTrue {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "completed")]
    Completed,
}

impl Default for StatusTrue {
    fn default() -> StatusTrue {
        Self::Active
    }
}
