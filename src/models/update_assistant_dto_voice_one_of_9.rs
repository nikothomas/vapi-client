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
pub struct UpdateAssistantDtoVoiceOneOf9 {
    #[serde(rename = "voiceId")]
    pub voice_id: models::PlayHtVoiceId,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Option<f64>>,
    /// A floating point number between 0, exclusive, and 2, inclusive. If equal to null or not provided, the model's default temperature will be used. The temperature parameter controls variance. Lower temperatures result in more predictable results, higher temperatures allow each run to vary more, so the voice may sound less like the baseline voice.
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f64>>,
    #[serde(rename = "emotion", skip_serializing_if = "Option::is_none")]
    pub emotion: Option<models::PlayHtVoiceEmotion>,
    /// A number between 1 and 6. Use lower numbers to reduce how unique your chosen voice will be compared to other voices.
    #[serde(rename = "voiceGuidance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_guidance: Option<Option<f64>>,
    /// A number between 1 and 30. Use lower numbers to to reduce how strong your chosen emotion will be. Higher numbers will create a very emotional performance.
    #[serde(rename = "styleGuidance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub style_guidance: Option<Option<f64>>,
    /// A number between 1 and 2. This number influences how closely the generated speech adheres to the input text. Use lower values to create more fluid speech, but with a higher chance of deviating from the input text. Higher numbers will make the generated speech more accurate to the input text, ensuring that the words spoken align closely with the provided text.
    #[serde(rename = "textGuidance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub text_guidance: Option<Option<f64>>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::PlayHtVoiceModel>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::PlayHtVoiceLanguage>,
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackPlan>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl UpdateAssistantDtoVoiceOneOf9 {
    pub fn new(voice_id: models::PlayHtVoiceId, provider: Provider) -> UpdateAssistantDtoVoiceOneOf9 {
        UpdateAssistantDtoVoiceOneOf9 {
            voice_id,
            speed: None,
            temperature: None,
            emotion: None,
            voice_guidance: None,
            style_guidance: None,
            text_guidance: None,
            model: None,
            language: None,
            chunk_plan: None,
            fallback_plan: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "playht")]
    Playht,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Playht
    }
}

