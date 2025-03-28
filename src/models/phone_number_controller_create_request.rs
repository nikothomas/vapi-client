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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(tag = "provider")]
pub enum PhoneNumberControllerCreateRequest {
    #[serde(rename = "byo-phone-number")]
    ByoPhoneNumber(models::CreateByoPhoneNumberDto),
    #[serde(rename = "twilio")]
    Twilio(models::CreateTwilioPhoneNumberDto),
    #[serde(rename = "vonage")]
    Vonage(models::CreateVonagePhoneNumberDto),
    #[serde(rename = "vapi")]
    Vapi(models::CreateVapiPhoneNumberDto),
}

impl Default for PhoneNumberControllerCreateRequest {
    fn default() -> Self {
        Self::ByoPhoneNumber(Default::default())
    }
}
