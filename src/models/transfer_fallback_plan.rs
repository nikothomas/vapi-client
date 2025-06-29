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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferFallbackPlan {
    #[serde(rename = "message")]
    pub message: models::TransferFallbackPlanMessage,
    /// This controls what happens after delivering the failure message to the customer. - true: End the call after delivering the failure message (default) - false: Keep the assistant on the call to continue handling the customer's request  @default true
    #[serde(rename = "endCallEnabled", skip_serializing_if = "Option::is_none")]
    pub end_call_enabled: Option<bool>,
}

impl TransferFallbackPlan {
    pub fn new(message: models::TransferFallbackPlanMessage) -> TransferFallbackPlan {
        TransferFallbackPlan {
            message,
            end_call_enabled: None,
        }
    }
}
