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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub enum PhoneNumber {
    #[serde(rename="byo-phone-number")]
    ByoPhoneNumber(Box<models::ByoPhoneNumber>),
    #[serde(rename="twilio")]
    Twilio(Box<models::TwilioPhoneNumber>),
    #[serde(rename="vonage")]
    Vonage(Box<models::VonagePhoneNumber>),
    #[serde(rename="vapi")]
    Vapi(Box<models::VapiPhoneNumber>),
    #[serde(rename="telnyx")]
    Telnyx(Box<models::TelnyxPhoneNumber>),
}

impl Default for PhoneNumber {
    fn default() -> Self {
        Self::ByoPhoneNumber(Default::default())
    }
}

/// This is the status of the phone number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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

