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
pub struct ClientMessageUserInterrupted {
    /// This is the type of the message. \"user-interrupted\" is sent when the user interrupts the assistant.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl ClientMessageUserInterrupted {
    pub fn new(r#type: Type) -> ClientMessageUserInterrupted {
        ClientMessageUserInterrupted {
            r#type,
        }
    }
}
/// This is the type of the message. \"user-interrupted\" is sent when the user interrupts the assistant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user-interrupted")]
    UserInterrupted,
}

impl Default for Type {
    fn default() -> Type {
        Self::UserInterrupted
    }
}

