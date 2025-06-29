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
pub struct SayPhoneNumberHookAction {
    /// This is the type of action - must be \"say\"
    #[serde(rename = "type")]
    pub r#type: TypeTrue,
    /// This is the message to say
    #[serde(rename = "exact")]
    pub exact: String,
}

impl SayPhoneNumberHookAction {
    pub fn new(r#type: TypeTrue, exact: String) -> SayPhoneNumberHookAction {
        SayPhoneNumberHookAction { r#type, exact }
    }
}
/// This is the type of action - must be \"say\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "say")]
    Say,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::Say
    }
}
