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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolMessageStart {
    /// This is an alternative to the `content` property. It allows to specify variants of the same content, one per language.  Usage: - If your assistants are multilingual, you can provide content for each language. - If you don't provide content for a language, the first item in the array will be automatically translated to the active language at that moment.  This will override the `content` property.
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<models::ToolMessageStartContentsInner>>,
    /// This message is triggered when the tool call starts.  This message is never triggered for async tools.  If this message is not provided, one of the default filler messages \"Hold on a sec\", \"One moment\", \"Just a sec\", \"Give me a moment\" or \"This'll just take a sec\" will be used.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is an optional boolean that if true, the tool call will only trigger after the message is spoken. Default is false.  @default false
    #[serde(rename = "blocking", skip_serializing_if = "Option::is_none")]
    pub blocking: Option<bool>,
    /// This is the content that the assistant says when this message is triggered.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// This is an optional array of conditions that the tool call arguments must meet in order for this message to be triggered.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::Condition>>,
}

impl ToolMessageStart {
    pub fn new(r#type: Type) -> ToolMessageStart {
        ToolMessageStart {
            contents: None,
            r#type,
            blocking: None,
            content: None,
            conditions: None,
        }
    }
}
/// This message is triggered when the tool call starts.  This message is never triggered for async tools.  If this message is not provided, one of the default filler messages \"Hold on a sec\", \"One moment\", \"Just a sec\", \"Give me a moment\" or \"This'll just take a sec\" will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "request-start")]
    RequestStart,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestStart
    }
}
