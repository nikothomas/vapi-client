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
pub struct OpenAiModel {
    /// This is the starting state for the conversation.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::OpenAiMessage>>,
    /// These are the tools that the assistant can use during the call. To use existing tools, use `toolIds`.  Both `tools` and `toolIds` can be used together.
    #[serde(rename = "tools", skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<models::AnyscaleModelToolsInner>>,
    /// These are the tools that the assistant can use during the call. To use transient tools, use `tools`.  Both `tools` and `toolIds` can be used together.
    #[serde(rename = "toolIds", skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    #[serde(rename = "knowledgeBase", skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<models::AnyscaleModelKnowledgeBase>,
    /// This is the ID of the knowledge base the model will use.
    #[serde(rename = "knowledgeBaseId", skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    /// This is the provider that will be used for the model.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the OpenAI model that will be used.
    #[serde(rename = "model")]
    pub model: Model,
    /// These are the fallback models that will be used if the primary model fails. This shouldn't be specified unless you have a specific reason to do so. Vapi will automatically find the fastest fallbacks that make sense.
    #[serde(rename = "fallbackModels", skip_serializing_if = "Option::is_none")]
    pub fallback_models: Option<Vec<FallbackModels>>,
    /// This is the temperature that will be used for calls. Default is 0 to leverage caching for lower latency.
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// This is the max number of tokens that the assistant will be allowed to generate in each turn of the conversation. Default is 250.
    #[serde(rename = "maxTokens", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<f64>,
    /// This determines whether we detect user's emotion while they speak and send it as an additional info to model.  Default `false` because the model is usually are good at understanding the user's emotion from text.  @default false
    #[serde(rename = "emotionRecognitionEnabled", skip_serializing_if = "Option::is_none")]
    pub emotion_recognition_enabled: Option<bool>,
    /// This sets how many turns at the start of the conversation to use a smaller, faster model from the same provider before switching to the primary model. Example, gpt-3.5-turbo if provider is openai.  Default is 0.  @default 0
    #[serde(rename = "numFastTurns", skip_serializing_if = "Option::is_none")]
    pub num_fast_turns: Option<f64>,
}

impl OpenAiModel {
    pub fn new(provider: Provider, model: Model) -> OpenAiModel {
        OpenAiModel {
            messages: None,
            tools: None,
            tool_ids: None,
            knowledge_base: None,
            knowledge_base_id: None,
            provider,
            model,
            fallback_models: None,
            temperature: None,
            max_tokens: None,
            emotion_recognition_enabled: None,
            num_fast_turns: None,
        }
    }
}
/// This is the provider that will be used for the model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "openai")]
    Openai,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Openai
    }
}
/// This is the OpenAI model that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "gpt-4.1")]
    Gpt4Period1,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt4Period1Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt4Period1Nano,
    #[serde(rename = "gpt-4.5-preview")]
    Gpt4Period5Preview,
    #[serde(rename = "chatgpt-4o-latest")]
    Chatgpt4oLatest,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o1-preview")]
    O1Preview,
    #[serde(rename = "o1-preview-2024-09-12")]
    O1Preview20240912,
    #[serde(rename = "o1-mini")]
    O1Mini,
    #[serde(rename = "o1-mini-2024-09-12")]
    O1Mini20240912,
    #[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
    Gpt4oRealtimePreview20241001,
    #[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
    Gpt4oRealtimePreview20241217,
    #[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
    Gpt4oMiniRealtimePreview20241217,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3Period5Turbo,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt3Period5Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt3Period5Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt3Period5Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt3Period5Turbo0613,
}

impl Default for Model {
    fn default() -> Model {
        Self::Gpt4Period1
    }
}
/// These are the fallback models that will be used if the primary model fails. This shouldn't be specified unless you have a specific reason to do so. Vapi will automatically find the fastest fallbacks that make sense.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FallbackModels {
    #[serde(rename = "gpt-4.1")]
    Gpt4Period1,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt4Period1Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt4Period1Nano,
    #[serde(rename = "gpt-4.5-preview")]
    Gpt4Period5Preview,
    #[serde(rename = "chatgpt-4o-latest")]
    Chatgpt4oLatest,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o1-preview")]
    O1Preview,
    #[serde(rename = "o1-preview-2024-09-12")]
    O1Preview20240912,
    #[serde(rename = "o1-mini")]
    O1Mini,
    #[serde(rename = "o1-mini-2024-09-12")]
    O1Mini20240912,
    #[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
    Gpt4oRealtimePreview20241001,
    #[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
    Gpt4oRealtimePreview20241217,
    #[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
    Gpt4oMiniRealtimePreview20241217,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3Period5Turbo,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt3Period5Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt3Period5Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt3Period5Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt3Period5Turbo0613,
}

impl Default for FallbackModels {
    fn default() -> FallbackModels {
        Self::Gpt4Period1
    }
}

