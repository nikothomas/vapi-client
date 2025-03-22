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

/// ClientMessageMessage : These are all the messages that can be sent to the client-side SDKs during the call. Configure the messages you'd like to receive in `assistant.clientMessages`.
/// These are all the messages that can be sent to the client-side SDKs during the call. Configure the messages you'd like to receive in `assistant.clientMessages`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(untagged)]
pub enum ClientMessageMessage {
    ClientMessageWorkflowNodeStarted(models::ClientMessageWorkflowNodeStarted),
    ClientMessageConversationUpdate(models::ClientMessageConversationUpdate),
    ClientMessageHang(models::ClientMessageHang),
    ClientMessageMetadata(models::ClientMessageMetadata),
    ClientMessageModelOutput(models::ClientMessageModelOutput),
    ClientMessageSpeechUpdate(models::ClientMessageSpeechUpdate),
    ClientMessageTranscript(models::ClientMessageTranscript),
    ClientMessageToolCalls(models::ClientMessageToolCalls),
    ClientMessageToolCallsResult(models::ClientMessageToolCallsResult),
    ClientMessageTransferUpdate(models::ClientMessageTransferUpdate),
    ClientMessageUserInterrupted(models::ClientMessageUserInterrupted),
    ClientMessageLanguageChangeDetected(models::ClientMessageLanguageChangeDetected),
    ClientMessageVoiceInput(models::ClientMessageVoiceInput),
}

impl Default for ClientMessageMessage {
    fn default() -> Self {
        Self::ClientMessageWorkflowNodeStarted(Default::default())
    }
}
/// This is the type of the message. \"workflow.node.started\" is sent when the active node changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "workflow.node.started")]
    WorkflowPeriodNodePeriodStarted,
    #[serde(rename = "conversation-update")]
    ConversationUpdate,
    #[serde(rename = "hang")]
    Hang,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "model-output")]
    ModelOutput,
    #[serde(rename = "speech-update")]
    SpeechUpdate,
    #[serde(rename = "transcript")]
    Transcript,
    #[serde(rename = "transcript[transcriptType=\"final\"]")]
    TranscriptLeftSquareBracketTranscriptTypeEqualDoubleQuoteFinalDoubleQuoteRightSquareBracket,
    #[serde(rename = "tool-calls")]
    ToolCalls,
    #[serde(rename = "tool-calls-result")]
    ToolCallsResult,
    #[serde(rename = "transfer-update")]
    TransferUpdate,
    #[serde(rename = "user-interrupted")]
    UserInterrupted,
    #[serde(rename = "language-change-detected")]
    LanguageChangeDetected,
    #[serde(rename = "voice-input")]
    VoiceInput,
}

impl Default for Type {
    fn default() -> Type {
        Self::WorkflowPeriodNodePeriodStarted
    }
}
/// This is the status of the speech update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Status {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "stopped")]
    Stopped,
}

impl Default for Status {
    fn default() -> Status {
        Self::Started
    }
}
/// This is the role for which the transcript is for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Assistant
    }
}
/// This is the type of the transcript.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum TranscriptType {
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "final")]
    Final,
}

impl Default for TranscriptType {
    fn default() -> TranscriptType {
        Self::Partial
    }
}
