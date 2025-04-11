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
pub struct ToolsListResponseItemOneOf2 {
    /// This determines if the tool is async.  If async, the assistant will move forward without waiting for your server to respond. This is useful if you just want to trigger something on your server.  If sync, the assistant will wait for your server to respond. This is useful if want assistant to respond with the result from your server.  Defaults to synchronous (`false`).
    #[serde(rename = "async", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<Option<bool>>,
    /// These are the messages that will be spoken to the user as the tool is running.  For some tools, this is auto-filled based on special fields like `tool.destinations`. For others like the function tool, these can be custom configured.
    #[serde(rename = "messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Option<Vec<models::FunctionToolMessagesItem>>>,
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
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::OpenAiFunction>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl ToolsListResponseItemOneOf2 {
    pub fn new(id: String, org_id: String, created_at: String, updated_at: String, r#type: Type) -> ToolsListResponseItemOneOf2 {
        ToolsListResponseItemOneOf2 {
            r#async: None,
            messages: None,
            id,
            org_id,
            created_at,
            updated_at,
            function: None,
            server: None,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "function")]
    Function,
}

impl Default for Type {
    fn default() -> Type {
        Self::Function
    }
}

