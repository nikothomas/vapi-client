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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallMessagesItem {
    UserMessage(Box<models::UserMessage>),
    SystemMessage(Box<models::SystemMessage>),
    BotMessage(Box<models::BotMessage>),
    ToolCallMessage(Box<models::ToolCallMessage>),
    ToolCallResultMessage(Box<models::ToolCallResultMessage>),
}

impl Default for CallMessagesItem {
    fn default() -> Self {
        Self::UserMessage(Default::default())
    }
}

