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
pub struct ToolsUpdateRequestOneOf12 {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<Option<bool>>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Option<Vec<models::UpdateGoogleSheetsRowAppendToolDtoMessagesItem>>>,
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::OpenAiFunction>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl ToolsUpdateRequestOneOf12 {
    pub fn new(r#type: Type) -> ToolsUpdateRequestOneOf12 {
        ToolsUpdateRequestOneOf12 {
            r#async: None,
            messages: None,
            function: None,
            server: None,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "google.sheets.row.append")]
    GooglePeriodSheetsPeriodRowPeriodAppend,
}

impl Default for Type {
    fn default() -> Type {
        Self::GooglePeriodSheetsPeriodRowPeriodAppend
    }
}

