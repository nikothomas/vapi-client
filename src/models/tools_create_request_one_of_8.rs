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
pub struct ToolsCreateRequestOneOf8 {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<Option<bool>>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Option<Vec<models::CreateComputerToolDtoMessagesItem>>>,
    /// The sub type of tool.
    #[serde(rename = "subType")]
    pub sub_type: SubType,
    /// The name of the tool, fixed to 'computer'
    #[serde(rename = "name")]
    pub name: Name,
    /// The display width in pixels
    #[serde(rename = "displayWidthPx")]
    pub display_width_px: f64,
    /// The display height in pixels
    #[serde(rename = "displayHeightPx")]
    pub display_height_px: f64,
    /// Optional display number
    #[serde(rename = "displayNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_number: Option<Option<f64>>,
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::OpenAiFunction>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl ToolsCreateRequestOneOf8 {
    pub fn new(sub_type: SubType, name: Name, display_width_px: f64, display_height_px: f64, r#type: Type) -> ToolsCreateRequestOneOf8 {
        ToolsCreateRequestOneOf8 {
            r#async: None,
            messages: None,
            sub_type,
            name,
            display_width_px,
            display_height_px,
            display_number: None,
            function: None,
            server: None,
            r#type,
        }
    }
}
/// The sub type of tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubType {
    #[serde(rename = "computer_20241022")]
    Computer20241022,
}

impl Default for SubType {
    fn default() -> SubType {
        Self::Computer20241022
    }
}
/// The name of the tool, fixed to 'computer'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "computer")]
    Computer,
}

impl Default for Name {
    fn default() -> Name {
        Self::Computer
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "computer")]
    Computer,
}

impl Default for Type {
    fn default() -> Type {
        Self::Computer
    }
}

