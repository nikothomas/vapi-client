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
pub struct OpenAiVoicemailDetectionPlan {
    /// This is how long should we listen in order to determine if we were sent to voicemail or not?  @default 15
    #[serde(rename = "voicemailExpectedDurationSeconds")]
    pub voicemail_expected_duration_seconds: f64,
}

impl OpenAiVoicemailDetectionPlan {
    pub fn new(voicemail_expected_duration_seconds: f64) -> OpenAiVoicemailDetectionPlan {
        OpenAiVoicemailDetectionPlan {
            voicemail_expected_duration_seconds,
        }
    }
}

