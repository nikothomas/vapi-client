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

/// ServerMessageMessage : These are all the messages that can be sent to your server before, after and during the call. Configure the messages you'd like to receive in `assistant.serverMessages`.  The server where the message is sent is determined by the following precedence order:  1. `tool.server.url` (if configured, and only for \"tool-calls\" message) 2. `assistant.serverUrl` (if configure) 3. `phoneNumber.serverUrl` (if configured) 4. `org.serverUrl` (if configured)
/// These are all the messages that can be sent to your server before, after and during the call. Configure the messages you'd like to receive in `assistant.serverMessages`.  The server where the message is sent is determined by the following precedence order:  1. `tool.server.url` (if configured, and only for \"tool-calls\" message) 2. `assistant.serverUrl` (if configure) 3. `phoneNumber.serverUrl` (if configured) 4. `org.serverUrl` (if configured)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessageMessage {
    ServerMessageAssistantRequest(Box<models::ServerMessageAssistantRequest>),
    ServerMessageConversationUpdate(Box<models::ServerMessageConversationUpdate>),
    ServerMessageEndOfCallReport(Box<models::ServerMessageEndOfCallReport>),
    ServerMessageHang(Box<models::ServerMessageHang>),
    ServerMessageKnowledgeBaseRequest(Box<models::ServerMessageKnowledgeBaseRequest>),
    ServerMessageModelOutput(Box<models::ServerMessageModelOutput>),
    ServerMessagePhoneCallControl(Box<models::ServerMessagePhoneCallControl>),
    ServerMessageSpeechUpdate(Box<models::ServerMessageSpeechUpdate>),
    ServerMessageStatusUpdate(Box<models::ServerMessageStatusUpdate>),
    ServerMessageToolCalls(Box<models::ServerMessageToolCalls>),
    ServerMessageTransferDestinationRequest(Box<models::ServerMessageTransferDestinationRequest>),
    ServerMessageTransferUpdate(Box<models::ServerMessageTransferUpdate>),
    ServerMessageTranscript(Box<models::ServerMessageTranscript>),
    ServerMessageUserInterrupted(Box<models::ServerMessageUserInterrupted>),
    ServerMessageLanguageChangeDetected(Box<models::ServerMessageLanguageChangeDetected>),
    ServerMessageVoiceInput(Box<models::ServerMessageVoiceInput>),
    ServerMessageVoiceRequest(Box<models::ServerMessageVoiceRequest>),
}

impl Default for ServerMessageMessage {
    fn default() -> Self {
        Self::ServerMessageAssistantRequest(Default::default())
    }
}

