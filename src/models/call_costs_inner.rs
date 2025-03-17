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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallCostsInner {
    TransportCost(Box<models::TransportCost>),
    TranscriberCost(Box<models::TranscriberCost>),
    ModelCost(Box<models::ModelCost>),
    VoiceCost(Box<models::VoiceCost>),
    VapiCost(Box<models::VapiCost>),
    VoicemailDetectionCost(Box<models::VoicemailDetectionCost>),
    AnalysisCost(Box<models::AnalysisCost>),
}

impl Default for CallCostsInner {
    fn default() -> Self {
        Self::TransportCost(Default::default())
    }
}
/// This is the type of cost, always 'transport' for this class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "transport")]
    Transport,
    #[serde(rename = "transcriber")]
    Transcriber,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "vapi")]
    Vapi,
    #[serde(rename = "voicemail-detection")]
    VoicemailDetection,
    #[serde(rename = "analysis")]
    Analysis,
}

impl Default for Type {
    fn default() -> Type {
        Self::Transport
    }
}
/// This is the provider that was used to detect the voicemail.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "openai")]
    Openai,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}
/// This is the sub type of the cost.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "overage")]
    Overage,
}

impl Default for SubType {
    fn default() -> SubType {
        Self::Normal
    }
}
/// This is the type of analysis performed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnalysisType {
    #[serde(rename = "summary")]
    Summary,
    #[serde(rename = "structuredData")]
    StructuredData,
    #[serde(rename = "successEvaluation")]
    SuccessEvaluation,
}

impl Default for AnalysisType {
    fn default() -> AnalysisType {
        Self::Summary
    }
}

