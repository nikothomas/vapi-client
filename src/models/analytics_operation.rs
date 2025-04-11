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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsOperation {
    #[serde(rename = "operation")]
    pub operation: models::AnalyticsOperationOperation,
    #[serde(rename = "column")]
    pub column: models::AnalyticsOperationColumn,
    /// This is the alias for column name returned. Defaults to `${operation}${column}`.
    #[serde(rename = "alias", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alias: Option<Option<String>>,
}

impl AnalyticsOperation {
    pub fn new(operation: models::AnalyticsOperationOperation, column: models::AnalyticsOperationColumn) -> AnalyticsOperation {
        AnalyticsOperation {
            operation,
            column,
            alias: None,
        }
    }
}

