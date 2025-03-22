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
pub enum PhoneNumberPaginatedResponseResultsInner {
    ByoPhoneNumber(models::ByoPhoneNumber),
    TwilioPhoneNumber(models::TwilioPhoneNumber),
    VonagePhoneNumber(models::VonagePhoneNumber),
    VapiPhoneNumber(models::VapiPhoneNumber),
}

impl Default for PhoneNumberPaginatedResponseResultsInner {
    fn default() -> Self {
        Self::ByoPhoneNumber(Default::default())
    }
}
/// This is to create free SIP phone numbers on Vapi.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Provider {
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Vapi
    }
}
/// This is the status of the phone number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "activating")]
    Activating,
    #[serde(rename = "blocked")]
    Blocked,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
