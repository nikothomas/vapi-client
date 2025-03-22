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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MonitorPlan {
    /// This determines whether the assistant's calls allow live listening. Defaults to true.  Fetch `call.monitor.listenUrl` to get the live listening URL.  @default true
    #[serde(rename = "listenEnabled", skip_serializing_if = "Option::is_none")]
    pub listen_enabled: Option<bool>,
    /// This determines whether the assistant's calls allow live control. Defaults to true.  Fetch `call.monitor.controlUrl` to get the live control URL.  To use, send any control message via a POST request to `call.monitor.controlUrl`. Here are the types of controls supported: https://docs.vapi.ai/api-reference/messages/client-inbound-message  @default true
    #[serde(rename = "controlEnabled", skip_serializing_if = "Option::is_none")]
    pub control_enabled: Option<bool>,
}

impl MonitorPlan {
    pub fn new() -> MonitorPlan {
        MonitorPlan {
            listen_enabled: None,
            control_enabled: None,
        }
    }
}
