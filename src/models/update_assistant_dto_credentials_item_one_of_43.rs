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
pub struct UpdateAssistantDtoCredentialsItemOneOf43 {
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl UpdateAssistantDtoCredentialsItemOneOf43 {
    pub fn new(provider: Provider) -> UpdateAssistantDtoCredentialsItemOneOf43 {
        UpdateAssistantDtoCredentialsItemOneOf43 {
            name: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "google.calendar.oauth2-client")]
    GooglePeriodCalendarPeriodOauth2Client,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::GooglePeriodCalendarPeriodOauth2Client
    }
}

