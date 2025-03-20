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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "rangeStart")]
    pub range_start: String,
    #[serde(rename = "rangeEnd")]
    pub range_end: String,
    #[serde(rename = "bill")]
    pub bill: f64,
    #[serde(rename = "billWithinBillingLimit")]
    pub bill_within_billing_limit: bool,
    #[serde(rename = "billDailyBreakdown")]
    pub bill_daily_breakdown: serde_json::Value,
    #[serde(rename = "callActive")]
    pub call_active: f64,
    #[serde(rename = "callActiveWithinConcurrencyLimit")]
    pub call_active_within_concurrency_limit: bool,
    #[serde(rename = "callMinutes")]
    pub call_minutes: f64,
    #[serde(rename = "callMinutesDailyBreakdown")]
    pub call_minutes_daily_breakdown: serde_json::Value,
    #[serde(rename = "callMinutesAverage")]
    pub call_minutes_average: f64,
    #[serde(rename = "callMinutesAverageDailyBreakdown")]
    pub call_minutes_average_daily_breakdown: serde_json::Value,
    #[serde(rename = "callCount")]
    pub call_count: f64,
    #[serde(rename = "callCountDailyBreakdown")]
    pub call_count_daily_breakdown: serde_json::Value,
}

impl Metrics {
    pub fn new(
        org_id: String,
        range_start: String,
        range_end: String,
        bill: f64,
        bill_within_billing_limit: bool,
        bill_daily_breakdown: serde_json::Value,
        call_active: f64,
        call_active_within_concurrency_limit: bool,
        call_minutes: f64,
        call_minutes_daily_breakdown: serde_json::Value,
        call_minutes_average: f64,
        call_minutes_average_daily_breakdown: serde_json::Value,
        call_count: f64,
        call_count_daily_breakdown: serde_json::Value,
    ) -> Metrics {
        Metrics {
            org_id,
            range_start,
            range_end,
            bill,
            bill_within_billing_limit,
            bill_daily_breakdown,
            call_active,
            call_active_within_concurrency_limit,
            call_minutes,
            call_minutes_daily_breakdown,
            call_minutes_average,
            call_minutes_average_daily_breakdown,
            call_count,
            call_count_daily_breakdown,
        }
    }
}
