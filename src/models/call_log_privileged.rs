/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
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
    #[serde(rename = "level")]
    pub level: models::CallLogPrivilegedLevel,
    /// This is the ISO 8601 date-time string of when the log was created.
    #[serde(rename = "time")]
    pub time: String,
}

impl CallLogPrivileged {
    pub fn new(call_id: String, org_id: String, log: String, level: models::CallLogPrivilegedLevel, time: String) -> CallLogPrivileged {
        CallLogPrivileged {
            call_id,
            org_id,
            log,
            level,
            time,
        }
    }
}

