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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(untagged)]
pub enum ClientMessageToolCallsToolWithToolCallListInner {
    FunctionToolWithToolCall(models::FunctionToolWithToolCall),
    GhlToolWithToolCall(models::GhlToolWithToolCall),
    MakeToolWithToolCall(models::MakeToolWithToolCall),
    BashToolWithToolCall(models::BashToolWithToolCall),
    ComputerToolWithToolCall(models::ComputerToolWithToolCall),
    TextEditorToolWithToolCall(models::TextEditorToolWithToolCall),
}

impl Default for ClientMessageToolCallsToolWithToolCallListInner {
    fn default() -> Self {
        Self::FunctionToolWithToolCall(Default::default())
    }
}
/// The type of tool. \"function\" for Function tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "ghl")]
    Ghl,
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "bash")]
    Bash,
    #[serde(rename = "computer")]
    Computer,
    #[serde(rename = "textEditor")]
    TextEditor,
}

impl Default for Type {
    fn default() -> Type {
        Self::Function
    }
}
/// The sub type of tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Name {
    #[serde(rename = "str_replace_editor")]
    StrReplaceEditor,
}

impl Default for Name {
    fn default() -> Name {
        Self::StrReplaceEditor
    }
}
