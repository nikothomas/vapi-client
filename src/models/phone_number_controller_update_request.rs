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
pub enum PhoneNumberControllerUpdateRequest {
    #[serde(rename = "byo-phone-number")]
    ByoPhoneNumber(models::UpdateByoPhoneNumberDto),
    #[serde(rename = "twilio")]
    Twilio(models::UpdateTwilioPhoneNumberDto),
    #[serde(rename = "vonage")]
    Vonage(models::UpdateVonagePhoneNumberDto),
    #[serde(rename = "vapi")]
    Vapi(models::UpdateVapiPhoneNumberDto),
}

impl Default for PhoneNumberControllerUpdateRequest {
    fn default() -> Self {
        Self::ByoPhoneNumber(Default::default())
    }
}
