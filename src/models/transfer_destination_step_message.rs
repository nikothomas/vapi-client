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

/// TransferDestinationStepMessage : This is spoken to the customer before connecting them to the destination.  Usage: - If this is not provided and transfer tool messages is not provided, default is \"Transferring the call now\". - If set to \"\", nothing is spoken. This is useful when you want to silently transfer. This is especially useful when transferring between assistants in a squad. In this scenario, you likely also want to set `assistant.firstMessageMode=assistant-speaks-first-with-model-generated-message` for the destination assistant.  This accepts a string or a ToolMessageStart class. Latter is useful if you want to specify multiple messages for different languages through the `contents` field.
/// This is spoken to the customer before connecting them to the destination.  Usage: - If this is not provided and transfer tool messages is not provided, default is \"Transferring the call now\". - If set to \"\", nothing is spoken. This is useful when you want to silently transfer. This is especially useful when transferring between assistants in a squad. In this scenario, you likely also want to set `assistant.firstMessageMode=assistant-speaks-first-with-model-generated-message` for the destination assistant.  This accepts a string or a ToolMessageStart class. Latter is useful if you want to specify multiple messages for different languages through the `contents` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransferDestinationStepMessage {
    String(String),
    CustomMessage(models::CustomMessage),
}

impl Default for TransferDestinationStepMessage {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
/// This is a custom message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "custom-message")]
    CustomMessage,
}

impl Default for Type {
    fn default() -> Type {
        Self::CustomMessage
    }
}

