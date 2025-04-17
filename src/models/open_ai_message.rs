use crate::models;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenAiMessage {
    /// Missing → None, present → Some(...)
    #[serde(default)]
    pub content: Option<String>,

    pub role: Role,

    /// Capture whatever payload comes back as "tool_calls"  
    /// (you can also replace `Value` with `Vec<models::ToolCall>` if you have a struct)
    #[serde(rename = "tool_calls", default)]
    pub tool_calls: Option<Value>,
}

impl OpenAiMessage {
    pub fn new(content: Option<String>, role: Role) -> Self {
        Self { content, role, tool_calls: None }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "tool")]
    Tool,
}

impl Default for Role {
    fn default() -> Role {
        Role::Assistant
    }
}
