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

/// TransferPlanMessage : This is the message the assistant will deliver to the destination party before connecting the customer.  Usage: - Used only when `mode` is `blind-transfer-add-summary-to-sip-header`, `warm-transfer-say-message` or `warm-transfer-wait-for-operator-to-speak-first-and-then-say-message`.
/// This is the message the assistant will deliver to the destination party before connecting the customer.  Usage: - Used only when `mode` is `blind-transfer-add-summary-to-sip-header`, `warm-transfer-say-message` or `warm-transfer-wait-for-operator-to-speak-first-and-then-say-message`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransferPlanMessage {
    String(String),
    CustomMessage(Box<models::CustomMessage>),
}

impl Default for TransferPlanMessage {
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

