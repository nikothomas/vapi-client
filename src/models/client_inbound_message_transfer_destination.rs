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

/// ClientInboundMessageTransferDestination : This is the destination to transfer the call to.
/// This is the destination to transfer the call to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientInboundMessageTransferDestination {
    TransferDestinationNumber(Box<models::TransferDestinationNumber>),
    TransferDestinationSip(Box<models::TransferDestinationSip>),
}

impl Default for ClientInboundMessageTransferDestination {
    fn default() -> Self {
        Self::TransferDestinationNumber(Default::default())
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

