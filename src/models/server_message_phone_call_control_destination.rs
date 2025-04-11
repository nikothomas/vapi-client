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

/// ServerMessagePhoneCallControlDestination : This is the destination to forward the call to if the request is \"forward\".
/// This is the destination to forward the call to if the request is \"forward\".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessagePhoneCallControlDestination {
    CreateTransferCallToolDtoDestinationsItemOneOf2(models::CreateTransferCallToolDtoDestinationsItemOneOf2),
    CreateTransferCallToolDtoDestinationsItemOneOf3(models::CreateTransferCallToolDtoDestinationsItemOneOf3),
}

impl Default for ServerMessagePhoneCallControlDestination {
    fn default() -> Self {
        Self::CreateTransferCallToolDtoDestinationsItemOneOf2(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "sip")]
    Sip,
}

impl Default for Type {
    fn default() -> Type {
        Self::Number
    }
}

