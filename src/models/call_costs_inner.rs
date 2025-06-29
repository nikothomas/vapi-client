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
    TransportCost(models::TransportCost),
    TranscriberCost(models::TranscriberCost),
    ModelCost(models::ModelCost),
    VoiceCost(models::VoiceCost),
    VapiCost(models::VapiCost),
    VoicemailDetectionCost(models::VoicemailDetectionCost),
    AnalysisCost(models::AnalysisCost),
    KnowledgeBaseCost(models::KnowledgeBaseCost),
}

impl Default for CallCostsInner {
    fn default() -> Self {
        Self::TransportCost(Default::default())
    }
}
/// This is the type of cost, always 'transport' for this class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
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
    #[serde(rename = "knowledge-base")]
    KnowledgeBase,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Transport
    }
}
/// This is the provider that was used to detect the voicemail.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderTrue {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "openai")]
    Openai,
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for ProviderTrue {
    fn default() -> ProviderTrue {
        Self::Twilio
    }
}
/// This is the sub type of the cost.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubTypeTrue {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "overage")]
    Overage,
}

impl Default for SubTypeTrue {
    fn default() -> SubTypeTrue {
        Self::Normal
    }
}
/// This is the type of analysis performed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnalysisTypeTrue {
    #[serde(rename = "summary")]
    Summary,
    #[serde(rename = "structuredData")]
    StructuredData,
    #[serde(rename = "successEvaluation")]
    SuccessEvaluation,
}

impl Default for AnalysisTypeTrue {
    fn default() -> AnalysisTypeTrue {
        Self::Summary
    }
}
