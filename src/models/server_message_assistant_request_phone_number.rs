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

/// ServerMessageAssistantRequestPhoneNumber : This is the phone number associated with the call.  This matches one of the following: - `call.phoneNumber`, - `call.phoneNumberId`.
/// This is the phone number associated with the call.  This matches one of the following: - `call.phoneNumber`, - `call.phoneNumberId`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessageAssistantRequestPhoneNumber {
    CreateByoPhoneNumberDto(Box<models::CreateByoPhoneNumberDto>),
    CreateTwilioPhoneNumberDto(Box<models::CreateTwilioPhoneNumberDto>),
    CreateVonagePhoneNumberDto(Box<models::CreateVonagePhoneNumberDto>),
    CreateVapiPhoneNumberDto(Box<models::CreateVapiPhoneNumberDto>),
}

impl Default for ServerMessageAssistantRequestPhoneNumber {
    fn default() -> Self {
        Self::CreateByoPhoneNumberDto(Default::default())
    }
}
/// This is to create free SIP phone numbers on Vapi.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Vapi
    }
}

