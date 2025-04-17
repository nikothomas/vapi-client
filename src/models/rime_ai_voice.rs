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
pub struct RimeAiVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "voiceId")]
    pub voice_id: models::RimeAiVoiceVoiceId,
    /// This is the model that will be used. Defaults to 'v1' when not specified.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// This is a flag that controls whether to add slight pauses using angle brackets. Example: “Hi. <200> I’d love to have a conversation with you.” adds a 200ms pause between the first and second sentences.
    #[serde(rename = "pauseBetweenBrackets", skip_serializing_if = "Option::is_none")]
    pub pause_between_brackets: Option<bool>,
    /// This is a flag that controls whether text inside brackets should be phonemized (converted to phonetic pronunciation) - Example: \"{h'El.o} World\" will pronounce \"Hello\" as expected.
    #[serde(rename = "phonemizeBetweenBrackets", skip_serializing_if = "Option::is_none")]
    pub phonemize_between_brackets: Option<bool>,
    /// This is a flag that controls whether to optimize for reduced latency in streaming. https://docs.rime.ai/api-reference/endpoint/websockets#param-reduce-latency
    #[serde(rename = "reduceLatency", skip_serializing_if = "Option::is_none")]
    pub reduce_latency: Option<bool>,
    /// This is a string that allows inline speed control using alpha notation. https://docs.rime.ai/api-reference/endpoint/websockets#param-inline-speed-alpha
    #[serde(rename = "inlineSpeedAlpha", skip_serializing_if = "Option::is_none")]
    pub inline_speed_alpha: Option<String>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    /// This is the plan for voice provider fallbacks in the event that the primary voice provider fails.
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackPlan>,
}

impl RimeAiVoice {
    pub fn new(provider: Provider, voice_id: models::RimeAiVoiceVoiceId) -> RimeAiVoice {
        RimeAiVoice {
            provider,
            voice_id,
            model: None,
            speed: None,
            pause_between_brackets: None,
            phonemize_between_brackets: None,
            reduce_latency: None,
            inline_speed_alpha: None,
            chunk_plan: None,
            fallback_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "rime-ai")]
    RimeAi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::RimeAi
    }
}
/// This is the model that will be used. Defaults to 'v1' when not specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "mist")]
    Mist,
    #[serde(rename = "mistv2")]
    Mistv2,
}

impl Default for Model {
    fn default() -> Model {
        Self::V1
    }
}

