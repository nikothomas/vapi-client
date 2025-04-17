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
pub struct CreateComputerToolDto {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::CreateDtmfToolDtoMessagesInner>>,
    /// The type of tool. \"computer\" for Computer tool.
    #[serde(rename = "type")]
    pub r#type: Type,
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
    #[serde(rename = "displayNumber", skip_serializing_if = "Option::is_none")]
    pub display_number: Option<f64>,
    /// This is the function definition of the tool.  For `endCall`, `transferCall`, and `dtmf` tools, this is auto-filled based on tool-specific fields like `tool.destinations`. But, even in those cases, you can provide a custom function definition for advanced use cases.  An example of an advanced use case is if you want to customize the message that's spoken for `endCall` tool. You can specify a function where it returns an argument \"reason\". Then, in `messages` array, you can have many \"request-complete\" messages. One of these messages will be triggered if the `messages[].conditions` matches the \"reason\" argument.
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::OpenAiFunction>,
    /// This is the server that will be hit when this tool is requested by the model.  All requests will be sent with the call object among other things. You can find more details in the Server URL documentation.  This overrides the serverUrl set on the org and the phoneNumber. Order of precedence: highest tool.server.url, then assistant.serverUrl, then phoneNumber.serverUrl, then org.serverUrl.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
}

impl CreateComputerToolDto {
    pub fn new(r#type: Type, sub_type: SubType, name: Name, display_width_px: f64, display_height_px: f64) -> CreateComputerToolDto {
        CreateComputerToolDto {
            r#async: None,
            messages: None,
            r#type,
            sub_type,
            name,
            display_width_px,
            display_height_px,
            display_number: None,
            function: None,
            server: None,
        }
    }
}
/// The type of tool. \"computer\" for Computer tool.
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

