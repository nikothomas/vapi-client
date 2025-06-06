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
pub struct GoogleCalendarCreateEventToolWithToolCall {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::CreateDtmfToolDtoMessagesInner>>,
    /// The type of tool. \"google.calendar.event.create\" for Google Calendar tool.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "toolCall")]
    pub tool_call: models::ToolCall,
    /// This is the function definition of the tool.  For `endCall`, `transferCall`, and `dtmf` tools, this is auto-filled based on tool-specific fields like `tool.destinations`. But, even in those cases, you can provide a custom function definition for advanced use cases.  An example of an advanced use case is if you want to customize the message that's spoken for `endCall` tool. You can specify a function where it returns an argument \"reason\". Then, in `messages` array, you can have many \"request-complete\" messages. One of these messages will be triggered if the `messages[].conditions` matches the \"reason\" argument.
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::OpenAiFunction>,
    /// This is the server that will be hit when this tool is requested by the model.  All requests will be sent with the call object among other things. You can find more details in the Server URL documentation.  This overrides the serverUrl set on the org and the phoneNumber. Order of precedence: highest tool.server.url, then assistant.serverUrl, then phoneNumber.serverUrl, then org.serverUrl.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
}

impl GoogleCalendarCreateEventToolWithToolCall {
    pub fn new(r#type: Type, tool_call: models::ToolCall) -> GoogleCalendarCreateEventToolWithToolCall {
        GoogleCalendarCreateEventToolWithToolCall {
            r#async: None,
            messages: None,
            r#type,
            tool_call,
            function: None,
            server: None,
        }
    }
}
/// The type of tool. \"google.calendar.event.create\" for Google Calendar tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "google.calendar.event.create")]
    GooglePeriodCalendarPeriodEventPeriodCreate,
}

impl Default for Type {
    fn default() -> Type {
        Self::GooglePeriodCalendarPeriodEventPeriodCreate
    }
}

