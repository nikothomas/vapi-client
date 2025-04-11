use crate::models;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SmartEndpointingEnabled {
    String(String),
    Bool(bool),
    Map(HashMap<String, Value>),
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartSpeakingPlan {
    /// This is how long assistant waits before speaking. Defaults to 0.4. ...
    #[serde(rename = "waitSeconds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wait_seconds: Option<Option<f64>>,

    /// Accepts either a string or a map of key-value pairs.
    #[serde(rename = "smartEndpointingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub smart_endpointing_enabled: Option<SmartEndpointingEnabled>,

    #[serde(rename = "smartEndpointingPlan", skip_serializing_if = "Option::is_none")]
    pub smart_endpointing_plan: Option<models::StartSpeakingPlanSmartEndpointingPlan>,

    /// These are the custom endpointing rules...
    #[serde(rename = "customEndpointingRules", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_endpointing_rules: Option<Option<Vec<models::StartSpeakingPlanCustomEndpointingRulesItem>>>,

    #[serde(rename = "transcriptionEndpointingPlan", skip_serializing_if = "Option::is_none")]
    pub transcription_endpointing_plan: Option<models::TranscriptionEndpointingPlan>,
}

impl StartSpeakingPlan {
    pub fn new() -> StartSpeakingPlan {
        StartSpeakingPlan {
            wait_seconds: None,
            smart_endpointing_enabled: None,
            smart_endpointing_plan: None,
            custom_endpointing_rules: None,
            transcription_endpointing_plan: None,
        }
    }
}
