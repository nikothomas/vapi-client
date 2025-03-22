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

/// CreateAssistantDtoModel : These are the options for the assistant's LLM.
/// These are the options for the assistant's LLM.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(untagged)]
pub enum CreateAssistantDtoModel {
    AnyscaleModel(models::AnyscaleModel),
    AnthropicModel(models::AnthropicModel),
    CustomLlmModel(models::CustomLlmModel),
    DeepInfraModel(models::DeepInfraModel),
    GoogleModel(models::GoogleModel),
    GroqModel(models::GroqModel),
    InflectionAiModel(models::InflectionAiModel),
    DeepSeekModel(models::DeepSeekModel),
    OpenAiModel(models::OpenAiModel),
    OpenRouterModel(models::OpenRouterModel),
    PerplexityAiModel(models::PerplexityAiModel),
    TogetherAiModel(models::TogetherAiModel),
    VapiModel(models::VapiModel),
    XaiModel(models::XaiModel),
}

impl Default for CreateAssistantDtoModel {
    fn default() -> Self {
        Self::AnyscaleModel(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "xai")]
    Xai,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Xai
    }
}
/// This is the name of the model. Ex. cognitivecomputations/dolphin-mixtral-8x7b
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Model {
    #[serde(rename = "grok-beta")]
    GrokBeta,
    #[serde(rename = "grok-2")]
    Grok2,
    #[serde(rename = "grok-3")]
    Grok3,
}

impl Default for Model {
    fn default() -> Model {
        Self::GrokBeta
    }
}
/// This determines whether metadata is sent in requests to the custom provider.  - `off` will not send any metadata. payload will look like `{ messages }` - `variable` will send `assistant.metadata` as a variable on the payload. payload will look like `{ messages, metadata }` - `destructured` will send `assistant.metadata` fields directly on the payload. payload will look like `{ messages, ...metadata }`  Further, `variable` and `destructured` will send `call`, `phoneNumber`, and `customer` objects in the payload.  Default is `variable`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum MetadataSendMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "variable")]
    Variable,
    #[serde(rename = "destructured")]
    Destructured,
}

impl Default for MetadataSendMode {
    fn default() -> MetadataSendMode {
        Self::Off
    }
}
/// These are the fallback models that will be used if the primary model fails. This shouldn't be specified unless you have a specific reason to do so. Vapi will automatically find the fastest fallbacks that make sense.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum FallbackModels {
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
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
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
        Self::Gpt4Period5Preview
    }
}
