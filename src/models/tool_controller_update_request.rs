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
#[serde(tag = "type")]
pub enum ToolControllerUpdateRequest {
    #[serde(rename = "dtmf")]
    Dtmf(models::UpdateDtmfToolDto),
    #[serde(rename = "endCall")]
    EndCall(models::UpdateEndCallToolDto),
    #[serde(rename = "function")]
    Function(models::UpdateFunctionToolDto),
    #[serde(rename = "ghl")]
    Ghl(models::UpdateGhlToolDto),
    #[serde(rename = "make")]
    Make(models::UpdateMakeToolDto),
    #[serde(rename = "transferCall")]
    TransferCall(models::UpdateTransferCallToolDto),
    #[serde(rename = "output")]
    Output(models::UpdateOutputToolDto),
    #[serde(rename = "bash")]
    Bash(models::UpdateBashToolDto),
    #[serde(rename = "computer")]
    Computer(models::UpdateComputerToolDto),
    #[serde(rename = "textEditor")]
    TextEditor(models::UpdateTextEditorToolDto),
    #[serde(rename = "query")]
    Query(models::UpdateQueryToolDto),
}

impl Default for ToolControllerUpdateRequest {
    fn default() -> Self {
        Self::Dtmf(Default::default())
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
