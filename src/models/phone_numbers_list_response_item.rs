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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhoneNumbersListResponseItem {
    PhoneNumbersListResponseItemOneOf(Box<models::PhoneNumbersListResponseItemOneOf>),
    PhoneNumbersListResponseItemOneOf1(Box<models::PhoneNumbersListResponseItemOneOf1>),
    PhoneNumbersListResponseItemOneOf2(Box<models::PhoneNumbersListResponseItemOneOf2>),
    PhoneNumbersListResponseItemOneOf3(Box<models::PhoneNumbersListResponseItemOneOf3>),
    PhoneNumbersListResponseItemOneOf4(Box<models::PhoneNumbersListResponseItemOneOf4>),
}

impl Default for PhoneNumbersListResponseItem {
    fn default() -> Self {
        Self::PhoneNumbersListResponseItemOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "telnyx")]
    Telnyx,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Telnyx
    }
}

