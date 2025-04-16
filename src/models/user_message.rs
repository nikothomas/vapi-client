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
pub struct UserMessage {
    /// The role of the user in the conversation.
    #[serde(rename = "role")]
    pub role: String,
    /// The message content from the user.
    #[serde(rename = "message")]
    pub message: String,
    /// The timestamp when the message was sent.
    #[serde(rename = "time")]
    pub time: f64,
    /// The timestamp when the message ended.
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// The number of seconds from the start of the conversation.
    #[serde(rename = "secondsFromStart")]
    pub seconds_from_start: f64,
    /// The duration of the message in seconds.
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<f64>>,
}

impl UserMessage {
    pub fn new(role: String, message: String, time: f64, end_time: f64, seconds_from_start: f64) -> UserMessage {
        UserMessage {
            role,
            message,
            time,
            end_time,
            seconds_from_start,
            duration: None,
        }
    }
}

