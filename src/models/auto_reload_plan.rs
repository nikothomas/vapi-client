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
pub struct AutoReloadPlan {
    /// This the amount of credits to reload.
    #[serde(rename = "credits")]
    pub credits: f64,
    /// This is the limit at which the reload is triggered.
    #[serde(rename = "threshold")]
    pub threshold: f64,
}

impl AutoReloadPlan {
    pub fn new(credits: f64, threshold: f64) -> AutoReloadPlan {
        AutoReloadPlan { credits, threshold }
    }
}
