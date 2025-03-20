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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputTool {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::CreateDtmfToolDtoMessagesInner>>,
    /// The type of tool. \"output\" for Output tool.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the unique identifier for the tool.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the organization that this tool belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the tool was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the tool was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the function definition of the tool.  For `endCall`, `transferCall`, and `dtmf` tools, this is auto-filled based on tool-specific fields like `tool.destinations`. But, even in those cases, you can provide a custom function definition for advanced use cases.  An example of an advanced use case is if you want to customize the message that's spoken for `endCall` tool. You can specify a function where it returns an argument \"reason\". Then, in `messages` array, you can have many \"request-complete\" messages. One of these messages will be triggered if the `messages[].conditions` matches the \"reason\" argument.
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::OpenAiFunction>,
    /// This is the server that will be hit when this tool is requested by the model.  All requests will be sent with the call object among other things. You can find more details in the Server URL documentation.  This overrides the serverUrl set on the org and the phoneNumber. Order of precedence: highest tool.server.url, then assistant.serverUrl, then phoneNumber.serverUrl, then org.serverUrl.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
}

impl OutputTool {
    pub fn new(
        r#type: Type,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
    ) -> OutputTool {
        OutputTool {
            r#async: None,
            messages: None,
            r#type,
            id,
            org_id,
            created_at,
            updated_at,
            function: None,
            server: None,
        }
    }
}
/// The type of tool. \"output\" for Output tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "output")]
    Output,
}

impl Default for Type {
    fn default() -> Type {
        Self::Output
    }
}
