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
pub struct UpdateAssistantDtoModelOneOf5 {
    /// This is the starting state for the conversation.
    #[serde(rename = "messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Option<Vec<models::OpenAiMessage>>>,
    /// These are the tools that the assistant can use during the call. To use existing tools, use `toolIds`.  Both `tools` and `toolIds` can be used together.
    #[serde(rename = "tools", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tools: Option<Option<Vec<models::DeepSeekModelToolsItem>>>,
    /// These are the tools that the assistant can use during the call. To use transient tools, use `tools`.  Both `tools` and `toolIds` can be used together.
    #[serde(rename = "toolIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "knowledgeBase", skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<models::CreateCustomKnowledgeBaseDto>,
    /// This is the ID of the knowledge base the model will use.
    #[serde(rename = "knowledgeBaseId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<Option<String>>,
    #[serde(rename = "model")]
    pub model: models::DeepSeekModelModel,
    /// This is the temperature that will be used for calls. Default is 0 to leverage caching for lower latency.
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f64>>,
    /// This is the max number of tokens that the assistant will be allowed to generate in each turn of the conversation. Default is 250.
    #[serde(rename = "maxTokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<Option<f64>>,
    /// This determines whether we detect user's emotion while they speak and send it as an additional info to model.  Default `false` because the model is usually are good at understanding the user's emotion from text.  @default false
    #[serde(rename = "emotionRecognitionEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub emotion_recognition_enabled: Option<Option<bool>>,
    /// This sets how many turns at the start of the conversation to use a smaller, faster model from the same provider before switching to the primary model. Example, gpt-3.5-turbo if provider is openai.  Default is 0.  @default 0
    #[serde(rename = "numFastTurns", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_fast_turns: Option<Option<f64>>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl UpdateAssistantDtoModelOneOf5 {
    pub fn new(model: models::DeepSeekModelModel, provider: Provider) -> UpdateAssistantDtoModelOneOf5 {
        UpdateAssistantDtoModelOneOf5 {
            messages: None,
            tools: None,
            tool_ids: None,
            knowledge_base: None,
            knowledge_base_id: None,
            model,
            temperature: None,
            max_tokens: None,
            emotion_recognition_enabled: None,
            num_fast_turns: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "deep-seek")]
    DeepSeek,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::DeepSeek
    }
}

