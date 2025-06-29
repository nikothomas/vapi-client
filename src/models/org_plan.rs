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
pub struct OrgPlan {
    #[serde(rename = "includedProviders", skip_serializing_if = "Option::is_none")]
    pub included_providers: Option<Vec<serde_json::Value>>,
    #[serde(rename = "includedMinutes", skip_serializing_if = "Option::is_none")]
    pub included_minutes: Option<f64>,
    #[serde(
        rename = "costPerOverageMinute",
        skip_serializing_if = "Option::is_none"
    )]
    pub cost_per_overage_minute: Option<f64>,
}

impl OrgPlan {
    pub fn new() -> OrgPlan {
        OrgPlan {
            included_providers: None,
            included_minutes: None,
            cost_per_overage_minute: None,
        }
    }
}
