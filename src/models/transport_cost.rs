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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TransportCost {
    /// This is the type of cost, always 'transport' for this class.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// This is the minutes of `transport` usage. This should match `call.endedAt` - `call.startedAt`.
    #[serde(rename = "minutes")]
    pub minutes: f64,
    /// This is the cost of the component in USD.
    #[serde(rename = "cost")]
    pub cost: f64,
}

impl TransportCost {
    pub fn new(r#type: Type, minutes: f64, cost: f64) -> TransportCost {
        TransportCost {
            r#type,
            provider: None,
            minutes,
            cost,
        }
    }
}
/// This is the type of cost, always 'transport' for this class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Type {
    #[serde(rename = "transport")]
    Transport,
}

impl Default for Type {
    fn default() -> Type {
        Self::Transport
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "vonage")]
    Vonage,
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}
