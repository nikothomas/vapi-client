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
pub struct CreateDtmfToolDtoMessagesItemOneOf3 {
    /// This is an alternative to the `content` property. It allows to specify variants of the same content, one per language.  Usage: - If your assistants are multilingual, you can provide content for each language. - If you don't provide content for a language, the first item in the array will be automatically translated to the active language at that moment.  This will override the `content` property.
    #[serde(rename = "contents", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Option<Vec<models::TextContent>>>,
    /// The number of milliseconds to wait for the server response before saying this message.
    #[serde(rename = "timingMilliseconds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timing_milliseconds: Option<Option<f64>>,
    /// This is the content that the assistant says when this message is triggered.
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    /// This is an optional array of conditions that the tool call arguments must meet in order for this message to be triggered.
    #[serde(rename = "conditions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Option<Vec<models::Condition>>>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl CreateDtmfToolDtoMessagesItemOneOf3 {
    pub fn new(r#type: Type) -> CreateDtmfToolDtoMessagesItemOneOf3 {
        CreateDtmfToolDtoMessagesItemOneOf3 {
            contents: None,
            timing_milliseconds: None,
            content: None,
            conditions: None,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "request-response-delayed")]
    RequestResponseDelayed,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestResponseDelayed
    }
}

