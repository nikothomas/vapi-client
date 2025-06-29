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
pub struct GoogleModel {
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
    pub knowledge_base: Option<models::CreateCustomKnowledgeBaseDto>,
    /// This is the ID of the knowledge base the model will use.
    #[serde(rename = "knowledgeBaseId", skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    /// This is the Google model that will be used.
    #[serde(rename = "model")]
    pub model: ModelTrue,
    #[serde(rename = "provider")]
    pub provider: ProviderTrue,
    /// This is the session configuration for the Gemini Flash 2.0 Multimodal Live API. Only applicable if the model `gemini-2.0-flash-realtime-exp` is selected.
    #[serde(rename = "realtimeConfig", skip_serializing_if = "Option::is_none")]
    pub realtime_config: Option<models::GoogleRealtimeConfig>,
    /// This is the temperature that will be used for calls. Default is 0 to leverage caching for lower latency.
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// This is the max number of tokens that the assistant will be allowed to generate in each turn of the conversation. Default is 250.
    #[serde(rename = "maxTokens", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<f64>,
    /// This determines whether we detect user's emotion while they speak and send it as an additional info to model.  Default `false` because the model is usually are good at understanding the user's emotion from text.  @default false
    #[serde(
        rename = "emotionRecognitionEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub emotion_recognition_enabled: Option<bool>,
    /// This sets how many turns at the start of the conversation to use a smaller, faster model from the same provider before switching to the primary model. Example, gpt-3.5-turbo if provider is openai.  Default is 0.  @default 0
    #[serde(rename = "numFastTurns", skip_serializing_if = "Option::is_none")]
    pub num_fast_turns: Option<f64>,
}

impl GoogleModel {
    pub fn new(model: ModelTrue, provider: ProviderTrue) -> GoogleModel {
        GoogleModel {
            messages: None,
            tools: None,
            tool_ids: None,
            knowledge_base: None,
            knowledge_base_id: None,
            model,
            provider,
            realtime_config: None,
            temperature: None,
            max_tokens: None,
            emotion_recognition_enabled: None,
            num_fast_turns: None,
        }
    }
}
/// This is the Google model that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ModelTrue {
    #[serde(rename = "gemini-2.5-pro-preview-05-06")]
    Gemini2Period5ProPreview0506,
    #[serde(rename = "gemini-2.5-flash-preview-05-20")]
    Gemini2Period5FlashPreview0520,
    #[serde(rename = "gemini-2.5-flash-preview-04-17")]
    Gemini2Period5FlashPreview0417,
    #[serde(rename = "gemini-2.0-flash-thinking-exp")]
    Gemini2Period0FlashThinkingExp,
    #[serde(rename = "gemini-2.0-pro-exp-02-05")]
    Gemini2Period0ProExp0205,
    #[serde(rename = "gemini-2.0-flash")]
    Gemini2Period0Flash,
    #[serde(rename = "gemini-2.0-flash-lite")]
    Gemini2Period0FlashLite,
    #[serde(rename = "gemini-2.0-flash-lite-preview-02-05")]
    Gemini2Period0FlashLitePreview0205,
    #[serde(rename = "gemini-2.0-flash-exp")]
    Gemini2Period0FlashExp,
    #[serde(rename = "gemini-2.0-flash-realtime-exp")]
    Gemini2Period0FlashRealtimeExp,
    #[serde(rename = "gemini-1.5-flash")]
    Gemini1Period5Flash,
    #[serde(rename = "gemini-1.5-flash-002")]
    Gemini1Period5Flash002,
    #[serde(rename = "gemini-1.5-pro")]
    Gemini1Period5Pro,
    #[serde(rename = "gemini-1.5-pro-002")]
    Gemini1Period5Pro002,
    #[serde(rename = "gemini-1.0-pro")]
    Gemini1Period0Pro,
}

impl Default for ModelTrue {
    fn default() -> ModelTrue {
        Self::Gemini2Period5ProPreview0506
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderTrue {
    #[serde(rename = "google")]
    Google,
}

impl Default for ProviderTrue {
    fn default() -> ProviderTrue {
        Self::Google
    }
}
