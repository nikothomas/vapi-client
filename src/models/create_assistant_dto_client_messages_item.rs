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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateAssistantDtoClientMessagesItem {
    #[serde(rename = "conversation-update")]
    ConversationUpdate,
    #[serde(rename = "function-call")]
    FunctionCall,
    #[serde(rename = "function-call-result")]
    FunctionCallResult,
    #[serde(rename = "hang")]
    Hang,
    #[serde(rename = "language-changed")]
    LanguageChanged,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "model-output")]
    ModelOutput,
    #[serde(rename = "speech-update")]
    SpeechUpdate,
    #[serde(rename = "status-update")]
    StatusUpdate,
    #[serde(rename = "transcript")]
    Transcript,
    #[serde(rename = "tool-calls")]
    ToolCalls,
    #[serde(rename = "tool-calls-result")]
    ToolCallsResult,
    #[serde(rename = "transfer-update")]
    TransferUpdate,
    #[serde(rename = "user-interrupted")]
    UserInterrupted,
    #[serde(rename = "voice-input")]
    VoiceInput,
    #[serde(rename = "workflow.node.started")]
    WorkflowPeriodNodePeriodStarted,

}

impl std::fmt::Display for CreateAssistantDtoClientMessagesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ConversationUpdate => write!(f, "conversation-update"),
            Self::FunctionCall => write!(f, "function-call"),
            Self::FunctionCallResult => write!(f, "function-call-result"),
            Self::Hang => write!(f, "hang"),
            Self::LanguageChanged => write!(f, "language-changed"),
            Self::Metadata => write!(f, "metadata"),
            Self::ModelOutput => write!(f, "model-output"),
            Self::SpeechUpdate => write!(f, "speech-update"),
            Self::StatusUpdate => write!(f, "status-update"),
            Self::Transcript => write!(f, "transcript"),
            Self::ToolCalls => write!(f, "tool-calls"),
            Self::ToolCallsResult => write!(f, "tool-calls-result"),
            Self::TransferUpdate => write!(f, "transfer-update"),
            Self::UserInterrupted => write!(f, "user-interrupted"),
            Self::VoiceInput => write!(f, "voice-input"),
            Self::WorkflowPeriodNodePeriodStarted => write!(f, "workflow.node.started"),
        }
    }
}

impl Default for CreateAssistantDtoClientMessagesItem {
    fn default() -> CreateAssistantDtoClientMessagesItem {
        Self::ConversationUpdate
    }
}

