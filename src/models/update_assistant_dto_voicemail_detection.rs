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

/// UpdateAssistantDtoVoicemailDetection : These are the settings to configure or disable voicemail detection. Alternatively, voicemail detection can be configured using the model.tools=[VoicemailTool]. This uses Twilio's built-in detection while the VoicemailTool relies on the model to detect if a voicemail was reached. You can use neither of them, one of them, or both of them. By default, Twilio built-in detection is enabled while VoicemailTool is not.
/// These are the settings to configure or disable voicemail detection. Alternatively, voicemail detection can be configured using the model.tools=[VoicemailTool]. This uses Twilio's built-in detection while the VoicemailTool relies on the model to detect if a voicemail was reached. You can use neither of them, one of them, or both of them. By default, Twilio built-in detection is enabled while VoicemailTool is not.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAssistantDtoVoicemailDetection {
    UpdateAssistantDtoVoicemailDetectionOneOf(Box<models::UpdateAssistantDtoVoicemailDetectionOneOf>),
    UpdateAssistantDtoVoicemailDetectionOneOf1(Box<models::UpdateAssistantDtoVoicemailDetectionOneOf1>),
    UpdateAssistantDtoVoicemailDetectionOneOf2(Box<models::UpdateAssistantDtoVoicemailDetectionOneOf2>),
}

impl Default for UpdateAssistantDtoVoicemailDetection {
    fn default() -> Self {
        Self::UpdateAssistantDtoVoicemailDetectionOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}

