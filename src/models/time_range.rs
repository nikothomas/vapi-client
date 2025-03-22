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
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct TimeRange {
    /// This is the time step for aggregations.  If not provided, defaults to returning for the entire time range.
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<Step>,
    /// This is the start date for the time range.  If not provided, defaults to the 7 days ago.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// This is the end date for the time range.  If not provided, defaults to now.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// This is the timezone you want to set for the query.  If not provided, defaults to UTC.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl TimeRange {
    pub fn new() -> TimeRange {
        TimeRange {
            step: None,
            start: None,
            end: None,
            timezone: None,
        }
    }
}
/// This is the time step for aggregations.  If not provided, defaults to returning for the entire time range.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Step {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "decade")]
    Decade,
    #[serde(rename = "century")]
    Century,
    #[serde(rename = "millennium")]
    Millennium,
}

impl Default for Step {
    fn default() -> Step {
        Self::Second
    }
}
