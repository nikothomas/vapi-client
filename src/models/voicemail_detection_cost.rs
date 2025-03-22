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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct VoicemailDetectionCost {
    /// This is the type of cost, always 'voicemail-detection' for this class.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the model that was used to perform the analysis.
    #[serde(rename = "model")]
    pub model: serde_json::Value,
    /// This is the provider that was used to detect the voicemail.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the number of prompt text tokens used in the voicemail detection.
    #[serde(rename = "promptTextTokens")]
    pub prompt_text_tokens: f64,
    /// This is the number of prompt audio tokens used in the voicemail detection.
    #[serde(rename = "promptAudioTokens")]
    pub prompt_audio_tokens: f64,
    /// This is the number of completion text tokens used in the voicemail detection.
    #[serde(rename = "completionTextTokens")]
    pub completion_text_tokens: f64,
    /// This is the number of completion audio tokens used in the voicemail detection.
    #[serde(rename = "completionAudioTokens")]
    pub completion_audio_tokens: f64,
    /// This is the cost of the component in USD.
    #[serde(rename = "cost")]
    pub cost: f64,
}

impl VoicemailDetectionCost {
    pub fn new(
        r#type: Type,
        model: serde_json::Value,
        provider: Provider,
        prompt_text_tokens: f64,
        prompt_audio_tokens: f64,
        completion_text_tokens: f64,
        completion_audio_tokens: f64,
        cost: f64,
    ) -> VoicemailDetectionCost {
        VoicemailDetectionCost {
            r#type,
            model,
            provider,
            prompt_text_tokens,
            prompt_audio_tokens,
            completion_text_tokens,
            completion_audio_tokens,
            cost,
        }
    }
}
/// This is the type of cost, always 'voicemail-detection' for this class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Type {
    #[serde(rename = "voicemail-detection")]
    VoicemailDetection,
}

impl Default for Type {
    fn default() -> Type {
        Self::VoicemailDetection
    }
}
/// This is the provider that was used to detect the voicemail.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "openai")]
    Openai,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}
