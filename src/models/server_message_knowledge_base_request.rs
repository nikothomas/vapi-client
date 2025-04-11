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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerMessageKnowledgeBaseRequest {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ServerMessageKnowledgeBaseRequestPhoneNumber>,
    /// This is the type of the message. \"knowledge-base-request\" is sent to request knowledge base documents. To enable, use `assistant.knowledgeBase.provider=custom-knowledge-base`.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// These are the messages that are going to be sent to the `model` right after the `knowledge-base-request` webhook completes.
    #[serde(rename = "messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Option<Vec<models::ServerMessageKnowledgeBaseRequestMessagesItem>>>,
    /// This is just `messages` formatted for OpenAI.
    #[serde(rename = "messagesOpenAIFormatted")]
    pub messages_open_ai_formatted: Vec<models::OpenAiMessage>,
    /// This is the timestamp of when the message was sent in milliseconds since Unix Epoch.
    #[serde(rename = "timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Option<f64>>,
    #[serde(rename = "artifact", skip_serializing_if = "Option::is_none")]
    pub artifact: Option<models::Artifact>,
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<models::CreateCustomerDto>,
    #[serde(rename = "call", skip_serializing_if = "Option::is_none")]
    pub call: Option<models::Call>,
}

impl ServerMessageKnowledgeBaseRequest {
    pub fn new(r#type: Type, messages_open_ai_formatted: Vec<models::OpenAiMessage>) -> ServerMessageKnowledgeBaseRequest {
        ServerMessageKnowledgeBaseRequest {
            phone_number: None,
            r#type,
            messages: None,
            messages_open_ai_formatted,
            timestamp: None,
            artifact: None,
            assistant: None,
            customer: None,
            call: None,
        }
    }
}
/// This is the type of the message. \"knowledge-base-request\" is sent to request knowledge base documents. To enable, use `assistant.knowledgeBase.provider=custom-knowledge-base`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "knowledge-base-request")]
    KnowledgeBaseRequest,
}

impl Default for Type {
    fn default() -> Type {
        Self::KnowledgeBaseRequest
    }
}

