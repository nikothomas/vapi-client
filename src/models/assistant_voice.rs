/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AssistantVoice : These are the options for the assistant's voice.
/// These are the options for the assistant's voice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantVoice {
    UpdateAssistantDtoVoiceOneOf(Box<models::UpdateAssistantDtoVoiceOneOf>),
    UpdateAssistantDtoVoiceOneOf1(Box<models::UpdateAssistantDtoVoiceOneOf1>),
    UpdateAssistantDtoVoiceOneOf2(Box<models::UpdateAssistantDtoVoiceOneOf2>),
    UpdateAssistantDtoVoiceOneOf3(Box<models::UpdateAssistantDtoVoiceOneOf3>),
    UpdateAssistantDtoVoiceOneOf4(Box<models::UpdateAssistantDtoVoiceOneOf4>),
    UpdateAssistantDtoVoiceOneOf5(Box<models::UpdateAssistantDtoVoiceOneOf5>),
    UpdateAssistantDtoVoiceOneOf6(Box<models::UpdateAssistantDtoVoiceOneOf6>),
    UpdateAssistantDtoVoiceOneOf7(Box<models::UpdateAssistantDtoVoiceOneOf7>),
    UpdateAssistantDtoVoiceOneOf8(Box<models::UpdateAssistantDtoVoiceOneOf8>),
    UpdateAssistantDtoVoiceOneOf9(Box<models::UpdateAssistantDtoVoiceOneOf9>),
    UpdateAssistantDtoVoiceOneOf10(Box<models::UpdateAssistantDtoVoiceOneOf10>),
    UpdateAssistantDtoVoiceOneOf11(Box<models::UpdateAssistantDtoVoiceOneOf11>),
    UpdateAssistantDtoVoiceOneOf12(Box<models::UpdateAssistantDtoVoiceOneOf12>),
    UpdateAssistantDtoVoiceOneOf13(Box<models::UpdateAssistantDtoVoiceOneOf13>),
}

impl Default for AssistantVoice {
    fn default() -> Self {
        Self::UpdateAssistantDtoVoiceOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Vapi
    }
}
/// Smallest AI voice model to use. Defaults to 'lightning' when not specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "lightning")]
    Lightning,
}

impl Default for Model {
    fn default() -> Model {
        Self::Lightning
    }
}

