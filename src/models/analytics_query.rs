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
pub struct AnalyticsQuery {
    /// This is the table you want to query.
    #[serde(rename = "table")]
    pub table: Table,
    /// This is the list of columns you want to group by.
    #[serde(rename = "groupBy", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupBy>>,
    /// This is the name of the query. This will be used to identify the query in the response.
    #[serde(rename = "name")]
    pub name: String,
    /// This is the time range for the query.
    #[serde(rename = "timeRange", skip_serializing_if = "Option::is_none")]
    pub time_range: Option<Box<models::TimeRange>>,
    /// This is the list of operations you want to perform.
    #[serde(rename = "operations")]
    pub operations: Vec<models::AnalyticsOperation>,
}

impl AnalyticsQuery {
    pub fn new(table: Table, name: String, operations: Vec<models::AnalyticsOperation>) -> AnalyticsQuery {
        AnalyticsQuery {
            table,
            group_by: None,
            name,
            time_range: None,
            operations,
        }
    }
}
/// This is the table you want to query.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Table {
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "subscription")]
    Subscription,
}

impl Default for Table {
    fn default() -> Table {
        Self::Call
    }
}
/// This is the list of columns you want to group by.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupBy {
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "assistantId")]
    AssistantId,
    #[serde(rename = "endedReason")]
    EndedReason,
    #[serde(rename = "analysis.successEvaluation")]
    AnalysisPeriodSuccessEvaluation,
    #[serde(rename = "status")]
    Status,
}

impl Default for GroupBy {
    fn default() -> GroupBy {
        Self::Type
    }
}

