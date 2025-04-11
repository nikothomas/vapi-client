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
pub enum GroqModelToolsItem {
    ToolsCreateRequestOneOf(Box<models::ToolsCreateRequestOneOf>),
    ToolsCreateRequestOneOf1(Box<models::ToolsCreateRequestOneOf1>),
    AnyscaleModelToolsItemOneOf(Box<models::AnyscaleModelToolsItemOneOf>),
    ToolsCreateRequestOneOf2(Box<models::ToolsCreateRequestOneOf2>),
    ToolsCreateRequestOneOf3(Box<models::ToolsCreateRequestOneOf3>),
    ToolsCreateRequestOneOf4(Box<models::ToolsCreateRequestOneOf4>),
    ToolsCreateRequestOneOf5(Box<models::ToolsCreateRequestOneOf5>),
    ToolsCreateRequestOneOf10(Box<models::ToolsCreateRequestOneOf10>),
    ToolsCreateRequestOneOf11(Box<models::ToolsCreateRequestOneOf11>),
    ToolsCreateRequestOneOf12(Box<models::ToolsCreateRequestOneOf12>),
    ToolsCreateRequestOneOf13(Box<models::ToolsCreateRequestOneOf13>),
    ToolsCreateRequestOneOf14(Box<models::ToolsCreateRequestOneOf14>),
}

impl Default for GroqModelToolsItem {
    fn default() -> Self {
        Self::ToolsCreateRequestOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dtmf")]
    Dtmf,
    #[serde(rename = "endCall")]
    EndCall,
    #[serde(rename = "voicemail")]
    Voicemail,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "ghl")]
    Ghl,
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "transferCall")]
    TransferCall,
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
}

impl Default for Type {
    fn default() -> Type {
        Self::Dtmf
    }
}

