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

/// CreateAssistantDtoTranscriber : These are the options for the assistant's transcriber.
/// These are the options for the assistant's transcriber.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantDtoTranscriber {
    UpdateAssistantDtoTranscriberOneOf(models::UpdateAssistantDtoTranscriberOneOf),
    UpdateAssistantDtoTranscriberOneOf1(models::UpdateAssistantDtoTranscriberOneOf1),
    UpdateAssistantDtoTranscriberOneOf2(models::UpdateAssistantDtoTranscriberOneOf2),
    UpdateAssistantDtoTranscriberOneOf3(models::UpdateAssistantDtoTranscriberOneOf3),
    UpdateAssistantDtoTranscriberOneOf4(models::UpdateAssistantDtoTranscriberOneOf4),
    UpdateAssistantDtoTranscriberOneOf5(models::UpdateAssistantDtoTranscriberOneOf5),
    UpdateAssistantDtoTranscriberOneOf6(models::UpdateAssistantDtoTranscriberOneOf6),
    UpdateAssistantDtoTranscriberOneOf7(models::UpdateAssistantDtoTranscriberOneOf7),
    UpdateAssistantDtoTranscriberOneOf8(models::UpdateAssistantDtoTranscriberOneOf8),
    UpdateAssistantDtoTranscriberOneOf9(models::UpdateAssistantDtoTranscriberOneOf9),
}

impl Default for CreateAssistantDtoTranscriber {
    fn default() -> Self {
        Self::UpdateAssistantDtoTranscriberOneOf(Default::default())
    }
}
/// 
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

