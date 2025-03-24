use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Example knowledge-base cost struct to match JSON like:
/// {
///   "cost": 0,
///   "type": "knowledge-base",
///   "model": {
///     "model": "gemini-1.5-flash",
///     "provider": "google"
///   },
///   "promptTokens": 0,
///   "completionTokens": 0
/// }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct KnowledgeBaseCost {
    /// This is the cost in USD.
    pub cost: f64,

    /// The cost type (always "knowledge-base" for this struct).
    #[serde(rename = "type")]
    pub cost_type: crate::models::call_costs_inner::Type,

    /// The model used for knowledge-base lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<KnowledgeBaseModel>,

    /// The number of tokens in the prompt, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptTokens: Option<u32>,

    /// The number of tokens in the completion, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completionTokens: Option<u32>,
}

/// If you need a more detailed model structure for knowledge-base usage, define it here:
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct KnowledgeBaseModel {
    pub model: String,
    pub provider: String,
}
