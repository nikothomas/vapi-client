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

/// DeepgramTranscriberModel : This is the Deepgram model that will be used. A list of models can be found here: https://developers.deepgram.com/docs/models-languages-overview
/// This is the Deepgram model that will be used. A list of models can be found here: https://developers.deepgram.com/docs/models-languages-overview
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeepgramTranscriberModel {
    // Nova-3 models
    #[serde(rename = "nova-3")]
    Nova3,
    #[serde(rename = "nova-3-general")]
    Nova3General,
    #[serde(rename = "nova-3-medical")]
    Nova3Medical,

    // Nova-2 models
    #[serde(rename = "nova-2")]
    Nova2,
    #[serde(rename = "nova-2-general")]
    Nova2General,
    #[serde(rename = "nova-2-meeting")]
    Nova2Meeting,
    #[serde(rename = "nova-2-phonecall")]
    Nova2Phonecall,
    #[serde(rename = "nova-2-finance")]
    Nova2Finance,
    #[serde(rename = "nova-2-conversationalai")]
    Nova2Conversationalai,
    #[serde(rename = "nova-2-voicemail")]
    Nova2Voicemail,
    #[serde(rename = "nova-2-video")]
    Nova2Video,
    #[serde(rename = "nova-2-medical")]
    Nova2Medical,
    #[serde(rename = "nova-2-drivethru")]
    Nova2Drivethru,
    #[serde(rename = "nova-2-automotive")]
    Nova2Automotive,
    #[serde(rename = "nova-2-atc")]
    Nova2Atc,

    // Nova (Nova-1) models
    #[serde(rename = "nova")]
    Nova,
    #[serde(rename = "nova-general")]
    NovaGeneral,
    #[serde(rename = "nova-phonecall")]
    NovaPhonecall,
    #[serde(rename = "nova-medical")]
    NovaMedical,

    // Enhanced models
    #[serde(rename = "enhanced")]
    Enhanced,
    #[serde(rename = "enhanced-general")]
    EnhancedGeneral,
    #[serde(rename = "enhanced-meeting")]
    EnhancedMeeting,
    #[serde(rename = "enhanced-phonecall")]
    EnhancedPhonecall,
    #[serde(rename = "enhanced-finance")]
    EnhancedFinance,

    // Base models
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "base-general")]
    BaseGeneral,
    #[serde(rename = "base-meeting")]
    BaseMeeting,
    #[serde(rename = "base-phonecall")]
    BasePhonecall,
    #[serde(rename = "base-finance")]
    BaseFinance,
    #[serde(rename = "base-conversationalai")]
    BaseConversationalai,
    #[serde(rename = "base-voicemail")]
    BaseVoicemail,
    #[serde(rename = "base-video")]
    BaseVideo,

    // Whisper models
    #[serde(rename = "whisper")]
    Whisper,
    #[serde(rename = "whisper-tiny")]
    WhisperTiny,
    #[serde(rename = "whisper-base")]
    WhisperBase,
    #[serde(rename = "whisper-small")]
    WhisperSmall,
    #[serde(rename = "whisper-medium")]
    WhisperMedium,
    #[serde(rename = "whisper-large")]
    WhisperLarge,

    // Custom model support - allows any string for custom models
    #[serde(untagged)]
    Custom(String),
}

impl Default for DeepgramTranscriberModel {
    fn default() -> Self {
        // Default to nova-2 as it's recommended for most use cases
        Self::Nova2
    }
}

impl DeepgramTranscriberModel {
    /// Returns the string representation of the model
    pub fn as_str(&self) -> &str {
        match self {
            Self::Nova3 => "nova-3",
            Self::Nova3General => "nova-3-general",
            Self::Nova3Medical => "nova-3-medical",
            Self::Nova2 => "nova-2",
            Self::Nova2General => "nova-2-general",
            Self::Nova2Meeting => "nova-2-meeting",
            Self::Nova2Phonecall => "nova-2-phonecall",
            Self::Nova2Finance => "nova-2-finance",
            Self::Nova2Conversationalai => "nova-2-conversationalai",
            Self::Nova2Voicemail => "nova-2-voicemail",
            Self::Nova2Video => "nova-2-video",
            Self::Nova2Medical => "nova-2-medical",
            Self::Nova2Drivethru => "nova-2-drivethru",
            Self::Nova2Automotive => "nova-2-automotive",
            Self::Nova2Atc => "nova-2-atc",
            Self::Nova => "nova",
            Self::NovaGeneral => "nova-general",
            Self::NovaPhonecall => "nova-phonecall",
            Self::NovaMedical => "nova-medical",
            Self::Enhanced => "enhanced",
            Self::EnhancedGeneral => "enhanced-general",
            Self::EnhancedMeeting => "enhanced-meeting",
            Self::EnhancedPhonecall => "enhanced-phonecall",
            Self::EnhancedFinance => "enhanced-finance",
            Self::Base => "base",
            Self::BaseGeneral => "base-general",
            Self::BaseMeeting => "base-meeting",
            Self::BasePhonecall => "base-phonecall",
            Self::BaseFinance => "base-finance",
            Self::BaseConversationalai => "base-conversationalai",
            Self::BaseVoicemail => "base-voicemail",
            Self::BaseVideo => "base-video",
            Self::Whisper => "whisper",
            Self::WhisperTiny => "whisper-tiny",
            Self::WhisperBase => "whisper-base",
            Self::WhisperSmall => "whisper-small",
            Self::WhisperMedium => "whisper-medium",
            Self::WhisperLarge => "whisper-large",
            Self::Custom(s) => s,
        }
    }
}

impl std::fmt::Display for DeepgramTranscriberModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
