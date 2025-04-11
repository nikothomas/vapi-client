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

/// ServerMessageResponseMessageResponse : This is the response that is expected from the server to the message.  Note: Most messages don't expect a response. Only \"assistant-request\", \"tool-calls\" and \"transfer-destination-request\" do.
/// This is the response that is expected from the server to the message.  Note: Most messages don't expect a response. Only \"assistant-request\", \"tool-calls\" and \"transfer-destination-request\" do.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessageResponseMessageResponse {
    ServerMessageResponseAssistantRequest(Box<models::ServerMessageResponseAssistantRequest>),
    ServerMessageResponseKnowledgeBaseRequest(Box<models::ServerMessageResponseKnowledgeBaseRequest>),
    ServerMessageResponseToolCalls(Box<models::ServerMessageResponseToolCalls>),
    ServerMessageResponseTransferDestinationRequest(Box<models::ServerMessageResponseTransferDestinationRequest>),
    ServerMessageResponseVoiceRequest(Box<models::ServerMessageResponseVoiceRequest>),
}

impl Default for ServerMessageResponseMessageResponse {
    fn default() -> Self {
        Self::ServerMessageResponseAssistantRequest(Default::default())
    }
}

