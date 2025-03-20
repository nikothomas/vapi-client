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
pub struct BotMessage {
    /// The role of the bot in the conversation.
    #[serde(rename = "role")]
    pub role: String,
    /// The message content from the bot.
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
    /// The source of the message.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The duration of the message in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

impl BotMessage {
    pub fn new(
        role: String,
        message: String,
        time: f64,
        end_time: f64,
        seconds_from_start: f64,
    ) -> BotMessage {
        BotMessage {
            role,
            message,
            time,
            end_time,
            seconds_from_start,
            source: None,
            duration: None,
        }
    }
}
