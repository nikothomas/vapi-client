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
pub struct AnalysisPlan {
    /// This is the plan for generating the summary of the call. This outputs to `call.analysis.summary`.
    #[serde(rename = "summaryPlan", skip_serializing_if = "Option::is_none")]
    pub summary_plan: Option<models::SummaryPlan>,
    /// This is the plan for generating the structured data from the call. This outputs to `call.analysis.structuredData`.
    #[serde(rename = "structuredDataPlan", skip_serializing_if = "Option::is_none")]
    pub structured_data_plan: Option<models::StructuredDataPlan>,
    /// This is the plan for generating the success evaluation of the call. This outputs to `call.analysis.successEvaluation`.
    #[serde(
        rename = "successEvaluationPlan",
        skip_serializing_if = "Option::is_none"
    )]
    pub success_evaluation_plan: Option<models::SuccessEvaluationPlan>,
}

impl AnalysisPlan {
    pub fn new() -> AnalysisPlan {
        AnalysisPlan {
            summary_plan: None,
            structured_data_plan: None,
            success_evaluation_plan: None,
        }
    }
}
