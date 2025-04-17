/*
 * Vapi API
 *
 * Voice AI for developers.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolBody {
    #[serde(rename="dtmf")]
    Dtmf(models::CreateDtmfToolDto),
    #[serde(rename="endCall")]
    EndCall(models::CreateEndCallToolDto),
    #[serde(rename="function")]
    Function(models::CreateFunctionToolDto),
    #[serde(rename="ghl")]
    Ghl(models::CreateGhlToolDto),
    #[serde(rename="make")]
    Make(models::CreateMakeToolDto),
    #[serde(rename="transferCall")]
    TransferCall(models::CreateTransferCallToolDto),
    #[serde(rename="output")]
    Output(models::CreateOutputToolDto),
    #[serde(rename="bash")]
    Bash(models::CreateBashToolDto),
    #[serde(rename="computer")]
    Computer(models::CreateComputerToolDto),
    #[serde(rename="textEditor")]
    TextEditor(models::CreateTextEditorToolDto),
    #[serde(rename="query")]
    Query(models::CreateQueryToolDto),
    #[serde(rename="google.calendar.event.create")]
    GooglePeriodCalendarPeriodEventPeriodCreate(models::CreateGoogleCalendarCreateEventToolDto),
    #[serde(rename="google.sheets.row.append")]
    GooglePeriodSheetsPeriodRowPeriodAppend(models::CreateGoogleSheetsRowAppendToolDto),
    #[serde(rename="google.calendar.availability.check")]
    GooglePeriodCalendarPeriodAvailabilityPeriodCheck(models::CreateGoogleCalendarCheckAvailabilityToolDto),
    #[serde(rename="slack.message.send")]
    SlackPeriodMessagePeriodSend(models::CreateSlackSendMessageToolDto),
    #[serde(rename="sms")]
    Sms(models::CreateSmsSendToolDto),
    #[serde(rename="mcp")]
    Mcp(models::CreateMcpToolDto),
}

impl Default for ToolBody {
    fn default() -> Self {
        Self::Dtmf(Default::default())
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

