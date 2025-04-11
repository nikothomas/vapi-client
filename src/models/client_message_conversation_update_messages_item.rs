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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientMessageConversationUpdateMessagesItem {
    UserMessage(models::UserMessage),
    SystemMessage(models::SystemMessage),
    BotMessage(models::BotMessage),
    ToolCallMessage(models::ToolCallMessage),
    ToolCallResultMessage(models::ToolCallResultMessage),
}

impl Default for ClientMessageConversationUpdateMessagesItem {
    fn default() -> Self {
        Self::UserMessage(Default::default())
    }
}

