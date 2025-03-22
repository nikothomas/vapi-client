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
pub struct GroqModel {
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
    /// This is the name of the model. Ex. cognitivecomputations/dolphin-mixtral-8x7b
    #[serde(rename = "model")]
    pub model: Model,
    #[serde(rename = "provider")]
    pub provider: Provider,
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

impl GroqModel {
    pub fn new(model: Model, provider: Provider) -> GroqModel {
        GroqModel {
            messages: None,
            tools: None,
            tool_ids: None,
            knowledge_base: None,
            knowledge_base_id: None,
            model,
            provider,
            temperature: None,
            max_tokens: None,
            emotion_recognition_enabled: None,
            num_fast_turns: None,
        }
    }
}
/// This is the name of the model. Ex. cognitivecomputations/dolphin-mixtral-8x7b
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Model {
    #[serde(rename = "deepseek-r1-distill-llama-70b")]
    DeepseekR1DistillLlama70b,
    #[serde(rename = "llama-3.3-70b-versatile")]
    Llama3Period370bVersatile,
    #[serde(rename = "llama-3.1-405b-reasoning")]
    Llama3Period1405bReasoning,
    #[serde(rename = "llama-3.1-70b-versatile")]
    Llama3Period170bVersatile,
    #[serde(rename = "llama-3.1-8b-instant")]
    Llama3Period18bInstant,
    #[serde(rename = "mixtral-8x7b-32768")]
    Mixtral8x7b32768,
    #[serde(rename = "llama3-8b-8192")]
    Llama38b8192,
    #[serde(rename = "llama3-70b-8192")]
    Llama370b8192,
    #[serde(rename = "gemma2-9b-it")]
    Gemma29bIt,
}

impl Default for Model {
    fn default() -> Model {
        Self::DeepseekR1DistillLlama70b
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "groq")]
    Groq,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Groq
    }
}
