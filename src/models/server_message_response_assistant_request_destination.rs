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

/// ServerMessageResponseAssistantRequestDestination : This is the destination to transfer the inbound call to. This will immediately transfer without using any assistants.  If this is sent, `assistantId`, `assistant`, `squadId`, and `squad` are ignored.
/// This is the destination to transfer the inbound call to. This will immediately transfer without using any assistants.  If this is sent, `assistantId`, `assistant`, `squadId`, and `squad` are ignored.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessageResponseAssistantRequestDestination {
    CreateTransferCallToolDtoDestinationsItemOneOf2(models::CreateTransferCallToolDtoDestinationsItemOneOf2),
    CreateTransferCallToolDtoDestinationsItemOneOf3(models::CreateTransferCallToolDtoDestinationsItemOneOf3),
}

impl Default for ServerMessageResponseAssistantRequestDestination {
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

