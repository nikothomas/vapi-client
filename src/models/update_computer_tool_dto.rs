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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateComputerToolDto {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::CreateDtmfToolDtoMessagesInner>>,
    /// The sub type of tool.
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,
    /// This is the function definition of the tool.  For `endCall`, `transferCall`, and `dtmf` tools, this is auto-filled based on tool-specific fields like `tool.destinations`. But, even in those cases, you can provide a custom function definition for advanced use cases.  An example of an advanced use case is if you want to customize the message that's spoken for `endCall` tool. You can specify a function where it returns an argument \"reason\". Then, in `messages` array, you can have many \"request-complete\" messages. One of these messages will be triggered if the `messages[].conditions` matches the \"reason\" argument.
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<Box<models::OpenAiFunction>>,
    /// This is the server that will be hit when this tool is requested by the model.  All requests will be sent with the call object among other things. You can find more details in the Server URL documentation.  This overrides the serverUrl set on the org and the phoneNumber. Order of precedence: highest tool.server.url, then assistant.serverUrl, then phoneNumber.serverUrl, then org.serverUrl.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<models::Server>>,
    /// The name of the tool, fixed to 'computer'
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// The display width in pixels
    #[serde(rename = "displayWidthPx", skip_serializing_if = "Option::is_none")]
    pub display_width_px: Option<f64>,
    /// The display height in pixels
    #[serde(rename = "displayHeightPx", skip_serializing_if = "Option::is_none")]
    pub display_height_px: Option<f64>,
    /// Optional display number
    #[serde(rename = "displayNumber", skip_serializing_if = "Option::is_none")]
    pub display_number: Option<f64>,
}

impl UpdateComputerToolDto {
    pub fn new() -> UpdateComputerToolDto {
        UpdateComputerToolDto {
            r#async: None,
            messages: None,
            sub_type: None,
            function: None,
            server: None,
            name: None,
            display_width_px: None,
            display_height_px: None,
            display_number: None,
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

