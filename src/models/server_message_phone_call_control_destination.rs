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

/// ServerMessagePhoneCallControlDestination : This is the destination to forward the call to if the request is \"forward\".
/// This is the destination to forward the call to if the request is \"forward\".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessagePhoneCallControlDestination {
    TransferDestinationNumber(models::TransferDestinationNumber),
    TransferDestinationSip(models::TransferDestinationSip),
}

impl Default for ServerMessagePhoneCallControlDestination {
    fn default() -> Self {
        Self::TransferDestinationNumber(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "sip")]
    Sip,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Number
    }
}
