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

/// ServerMessageAssistantRequestPhoneNumber : This is the phone number associated with the call.  This matches one of the following: - `call.phoneNumber`, - `call.phoneNumberId`.
/// This is the phone number associated with the call.  This matches one of the following: - `call.phoneNumber`, - `call.phoneNumberId`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(untagged)]
pub enum ServerMessageAssistantRequestPhoneNumber {
    CreateByoPhoneNumberDto(models::CreateByoPhoneNumberDto),
    CreateTwilioPhoneNumberDto(models::CreateTwilioPhoneNumberDto),
    CreateVonagePhoneNumberDto(models::CreateVonagePhoneNumberDto),
    CreateVapiPhoneNumberDto(models::CreateVapiPhoneNumberDto),
}

impl Default for ServerMessageAssistantRequestPhoneNumber {
    fn default() -> Self {
        Self::CreateByoPhoneNumberDto(Default::default())
    }
}
/// This is to create free SIP phone numbers on Vapi.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Vapi
    }
}
