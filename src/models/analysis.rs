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
pub struct Analysis {
    /// This is the summary of the call. Customize by setting `assistant.analysisPlan.summaryPrompt`.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// This is the structured data extracted from the call. Customize by setting `assistant.analysisPlan.structuredDataPrompt` and/or `assistant.analysisPlan.structuredDataSchema`.
    #[serde(rename = "structuredData", skip_serializing_if = "Option::is_none")]
    pub structured_data: Option<serde_json::Value>,
    /// This is the evaluation of the call. Customize by setting `assistant.analysisPlan.successEvaluationPrompt` and/or `assistant.analysisPlan.successEvaluationRubric`.
    #[serde(rename = "successEvaluation", skip_serializing_if = "Option::is_none")]
    pub success_evaluation: Option<String>,
}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis {
            summary: None,
            structured_data: None,
            success_evaluation: None,
        }
    }
}
