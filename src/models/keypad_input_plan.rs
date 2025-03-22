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
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct KeypadInputPlan {
    /// This keeps track of whether the user has enabled keypad input. By default, it is off.  @default false
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// This is the time in seconds to wait before processing the input. If the input is not received within this time, the input will be ignored. If set to \"off\", the input will be processed when the user enters a delimiter or immediately if no delimiter is used.  @default 2
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<f64>,
    /// This is the delimiter(s) that will be used to process the input. Can be '#', '*', or an empty array.
    #[serde(rename = "delimiters", skip_serializing_if = "Option::is_none")]
    pub delimiters: Option<Delimiters>,
}

impl KeypadInputPlan {
    pub fn new() -> KeypadInputPlan {
        KeypadInputPlan {
            enabled: None,
            timeout_seconds: None,
            delimiters: None,
        }
    }
}
/// This is the delimiter(s) that will be used to process the input. Can be '#', '*', or an empty array.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Delimiters {
    #[serde(rename = "#")]
    Hash,
    #[serde(rename = "*")]
    Star,
    #[serde(rename = "")]
    Empty,
}

impl Default for Delimiters {
    fn default() -> Delimiters {
        Self::Hash
    }
}
