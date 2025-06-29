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
pub struct CallLogPrivileged {
    /// This is the unique identifier for the call.
    #[serde(rename = "callId")]
    pub call_id: String,
    /// This is the unique identifier for the org that this call log belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the log message associated with the call.
    #[serde(rename = "log")]
    pub log: String,
    /// This is the level of the log message.
    #[serde(rename = "level")]
    pub level: LevelTrue,
    /// This is the ISO 8601 date-time string of when the log was created.
    #[serde(rename = "time")]
    pub time: String,
}

impl CallLogPrivileged {
    pub fn new(
        call_id: String,
        org_id: String,
        log: String,
        level: LevelTrue,
        time: String,
    ) -> CallLogPrivileged {
        CallLogPrivileged {
            call_id,
            org_id,
            log,
            level,
            time,
        }
    }
}
/// This is the level of the log message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LevelTrue {
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "LOG")]
    Log,
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "CHECKPOINT")]
    Checkpoint,
}

impl Default for LevelTrue {
    fn default() -> LevelTrue {
        Self::Info
    }
}
