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
pub struct CostBreakdown {
    /// This is the cost of the transport provider, like Twilio or Vonage.
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<f64>,
    /// This is the cost of the speech-to-text service.
    #[serde(rename = "stt", skip_serializing_if = "Option::is_none")]
    pub stt: Option<f64>,
    /// This is the cost of the language model.
    #[serde(rename = "llm", skip_serializing_if = "Option::is_none")]
    pub llm: Option<f64>,
    /// This is the cost of the text-to-speech service.
    #[serde(rename = "tts", skip_serializing_if = "Option::is_none")]
    pub tts: Option<f64>,
    /// This is the cost of Vapi.
    #[serde(rename = "vapi", skip_serializing_if = "Option::is_none")]
    pub vapi: Option<f64>,
    /// This is the cost of chat interactions.
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<f64>,
    /// This is the total cost of the call.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// This is the LLM prompt tokens used for the call.
    #[serde(rename = "llmPromptTokens", skip_serializing_if = "Option::is_none")]
    pub llm_prompt_tokens: Option<f64>,
    /// This is the LLM completion tokens used for the call.
    #[serde(
        rename = "llmCompletionTokens",
        skip_serializing_if = "Option::is_none"
    )]
    pub llm_completion_tokens: Option<f64>,
    /// This is the TTS characters used for the call.
    #[serde(rename = "ttsCharacters", skip_serializing_if = "Option::is_none")]
    pub tts_characters: Option<f64>,
    /// This is the cost of the analysis.
    #[serde(
        rename = "analysisCostBreakdown",
        skip_serializing_if = "Option::is_none"
    )]
    pub analysis_cost_breakdown: Option<models::AnalysisCostBreakdown>,
}

impl CostBreakdown {
    pub fn new() -> CostBreakdown {
        CostBreakdown {
            transport: None,
            stt: None,
            llm: None,
            tts: None,
            vapi: None,
            chat: None,
            total: None,
            llm_prompt_tokens: None,
            llm_completion_tokens: None,
            tts_characters: None,
            analysis_cost_breakdown: None,
        }
    }
}
