/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ServerMessageTransferDestinationRequestPhoneNumber : This is the phone number associated with the call.  This matches one of the following: - `call.phoneNumber`, - `call.phoneNumberId`.
/// This is the phone number associated with the call.  This matches one of the following: - `call.phoneNumber`, - `call.phoneNumberId`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessageTransferDestinationRequestPhoneNumber {
    PhoneNumbersCreateRequestOneOf(Box<models::PhoneNumbersCreateRequestOneOf>),
    PhoneNumbersCreateRequestOneOf1(Box<models::PhoneNumbersCreateRequestOneOf1>),
    PhoneNumbersCreateRequestOneOf2(Box<models::PhoneNumbersCreateRequestOneOf2>),
    PhoneNumbersCreateRequestOneOf3(Box<models::PhoneNumbersCreateRequestOneOf3>),
}

impl Default for ServerMessageTransferDestinationRequestPhoneNumber {
    fn default() -> Self {
        Self::PhoneNumbersCreateRequestOneOf(Default::default())
    }
}
/// 
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

