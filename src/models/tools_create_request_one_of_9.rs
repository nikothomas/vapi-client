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
pub struct ToolsCreateRequestOneOf9 {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<Option<bool>>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Option<Vec<models::CreateTextEditorToolDtoMessagesItem>>>,
    /// The sub type of tool.
    #[serde(rename = "subType")]
    pub sub_type: SubType,
    /// The name of the tool, fixed to 'str_replace_editor'
    #[serde(rename = "name")]
    pub name: Name,
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::OpenAiFunction>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl ToolsCreateRequestOneOf9 {
    pub fn new(sub_type: SubType, name: Name, r#type: Type) -> ToolsCreateRequestOneOf9 {
        ToolsCreateRequestOneOf9 {
            r#async: None,
            messages: None,
            sub_type,
            name,
            function: None,
            server: None,
            r#type,
        }
    }
}
/// The sub type of tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubType {
    #[serde(rename = "text_editor_20241022")]
    TextEditor20241022,
}

impl Default for SubType {
    fn default() -> SubType {
        Self::TextEditor20241022
    }
}
/// The name of the tool, fixed to 'str_replace_editor'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "str_replace_editor")]
    StrReplaceEditor,
}

impl Default for Name {
    fn default() -> Name {
        Self::StrReplaceEditor
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "textEditor")]
    TextEditor,
}

impl Default for Type {
    fn default() -> Type {
        Self::TextEditor
    }
}

