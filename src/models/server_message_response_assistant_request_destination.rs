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

/// ServerMessageResponseAssistantRequestDestination : This is the destination to transfer the inbound call to. This will immediately transfer without using any assistants.  If this is sent, `assistantId`, `assistant`, `squadId`, and `squad` are ignored.
/// This is the destination to transfer the inbound call to. This will immediately transfer without using any assistants.  If this is sent, `assistantId`, `assistant`, `squadId`, and `squad` are ignored.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessageResponseAssistantRequestDestination {
    TransferDestinationNumber(models::TransferDestinationNumber),
    TransferDestinationSip(models::TransferDestinationSip),
}

impl Default for ServerMessageResponseAssistantRequestDestination {
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
