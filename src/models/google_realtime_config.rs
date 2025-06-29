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
pub struct GoogleRealtimeConfig {
    /// This is the nucleus sampling parameter that controls the cumulative probability of tokens considered during text generation. Only applicable with the Gemini Flash 2.0 Multimodal Live API.
    #[serde(rename = "topP", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// This is the top-k sampling parameter that limits the number of highest probability tokens considered during text generation. Only applicable with the Gemini Flash 2.0 Multimodal Live API.
    #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f64>,
    /// This is the presence penalty parameter that influences the model's likelihood to repeat information by penalizing tokens based on their presence in the text. Only applicable with the Gemini Flash 2.0 Multimodal Live API.
    #[serde(rename = "presencePenalty", skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// This is the frequency penalty parameter that influences the model's likelihood to repeat tokens by penalizing them based on their frequency in the text. Only applicable with the Gemini Flash 2.0 Multimodal Live API.
    #[serde(rename = "frequencyPenalty", skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// This is the speech configuration object that defines the voice settings to be used for the model's speech output. Only applicable with the Gemini Flash 2.0 Multimodal Live API.
    #[serde(rename = "speechConfig", skip_serializing_if = "Option::is_none")]
    pub speech_config: Option<models::GeminiMultimodalLiveSpeechConfig>,
}

impl GoogleRealtimeConfig {
    pub fn new() -> GoogleRealtimeConfig {
        GoogleRealtimeConfig {
            top_p: None,
            top_k: None,
            presence_penalty: None,
            frequency_penalty: None,
            speech_config: None,
        }
    }
}
