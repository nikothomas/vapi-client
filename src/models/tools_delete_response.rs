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
pub enum ToolsDeleteResponse {
    ToolsListResponseItemOneOf(models::ToolsListResponseItemOneOf),
    ToolsListResponseItemOneOf1(models::ToolsListResponseItemOneOf1),
    ToolsListResponseItemOneOf2(models::ToolsListResponseItemOneOf2),
    ToolsListResponseItemOneOf3(models::ToolsListResponseItemOneOf3),
    ToolsListResponseItemOneOf4(models::ToolsListResponseItemOneOf4),
    ToolsListResponseItemOneOf5(models::ToolsListResponseItemOneOf5),
    ToolsListResponseItemOneOf6(models::ToolsListResponseItemOneOf6),
    ToolsListResponseItemOneOf7(models::ToolsListResponseItemOneOf7),
    ToolsListResponseItemOneOf8(models::ToolsListResponseItemOneOf8),
    ToolsListResponseItemOneOf9(models::ToolsListResponseItemOneOf9),
    ToolsListResponseItemOneOf10(models::ToolsListResponseItemOneOf10),
    ToolsListResponseItemOneOf11(models::ToolsListResponseItemOneOf11),
    ToolsListResponseItemOneOf12(models::ToolsListResponseItemOneOf12),
    ToolsListResponseItemOneOf13(models::ToolsListResponseItemOneOf13),
    ToolsListResponseItemOneOf14(models::ToolsListResponseItemOneOf14),
    ToolsListResponseItemOneOf15(models::ToolsListResponseItemOneOf15),
}

impl Default for ToolsDeleteResponse {
    fn default() -> Self {
        Self::ToolsListResponseItemOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dtmf")]
    Dtmf,
    #[serde(rename = "endCall")]
    EndCall,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "ghl")]
    Ghl,
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "transferCall")]
    TransferCall,
    #[serde(rename = "output")]
    Output,
    #[serde(rename = "bash")]
    Bash,
    #[serde(rename = "computer")]
    Computer,
    #[serde(rename = "textEditor")]
    TextEditor,
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "google.calendar.event.create")]
    GooglePeriodCalendarPeriodEventPeriodCreate,
    #[serde(rename = "google.sheets.row.append")]
    GooglePeriodSheetsPeriodRowPeriodAppend,
    #[serde(rename = "google.calendar.availability.check")]
    GooglePeriodCalendarPeriodAvailabilityPeriodCheck,
    #[serde(rename = "slack.message.send")]
    SlackPeriodMessagePeriodSend,
    #[serde(rename = "mcp")]
    Mcp,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dtmf
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

