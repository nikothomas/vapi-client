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
pub struct AnalyticsQueryResult {
    /// This is the unique key for the query.
    #[serde(rename = "name")]
    pub name: String,
    /// This is the time range for the query.
    #[serde(rename = "timeRange")]
    pub time_range: models::TimeRange,
    /// This is the result of the query, a list of unique groups with result of their aggregations.  Example: \"result\": [   { \"date\": \"2023-01-01\", \"assistantId\": \"123\", \"endedReason\": \"customer-ended-call\", \"sumDuration\": 120, \"avgCost\": 10.5 },   { \"date\": \"2023-01-02\", \"assistantId\": \"123\", \"endedReason\": \"customer-did-not-give-microphone-permission\", \"sumDuration\": 0, \"avgCost\": 0 },   // Additional results ]
    #[serde(rename = "result")]
    pub result: Vec<serde_json::Value>,
}

impl AnalyticsQueryResult {
    pub fn new(
        name: String,
        time_range: models::TimeRange,
        result: Vec<serde_json::Value>,
    ) -> AnalyticsQueryResult {
        AnalyticsQueryResult {
            name,
            time_range,
            result,
        }
    }
}
