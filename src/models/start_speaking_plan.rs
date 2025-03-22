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
pub struct StartSpeakingPlan {
    /// This is how long assistant waits before speaking. Defaults to 0.4.  This is the minimum it will wait but if there is latency is the pipeline, this minimum will be exceeded. This is intended as a stopgap in case the pipeline is moving too fast.  Example: - If model generates tokens and voice generates bytes within 100ms, the pipeline still waits 300ms before outputting speech.  Usage: - If the customer is taking long pauses, set this to a higher value. - If the assistant is accidentally jumping in too much, set this to a higher value.  @default 0.4
    #[serde(rename = "waitSeconds", skip_serializing_if = "Option::is_none")]
    pub wait_seconds: Option<f64>,
    /// This determines if a customer speech is considered done (endpointing) using a Vapi custom-trained model on customer's speech. This is good for middle-of-thought detection.  Once an endpoint is triggered, the request is sent to `assistant.model`.  Usage: - If your conversations are long-form and you want assistant to wait smartly even if customer pauses for a bit to think, you can use this instead.  This overrides `transcriptionEndpointingPlan`.  @default false
    #[serde(
        rename = "smartEndpointingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub smart_endpointing_enabled: Option<bool>,
    /// These are the custom endpointing rules to set an endpointing timeout based on a regex on the customer's speech or the assistant's last message.  Usage: - If you have yes/no questions like \"are you interested in a loan?\", you can set a shorter timeout. - If you have questions where the customer may pause to look up information like \"what's my account number?\", you can set a longer timeout. - If you want to wait longer while customer is enumerating a list of numbers, you can set a longer timeout.  These override `transcriptionEndpointingPlan` and `smartEndpointingEnabled` when a rule is matched.  The rules are evaluated in order and the first one that matches will be used.  @default []
    #[serde(
        rename = "customEndpointingRules",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_endpointing_rules: Option<Vec<models::StartSpeakingPlanCustomEndpointingRulesInner>>,
    /// This determines how a customer speech is considered done (endpointing) using the transcription of customer's speech.  Once an endpoint is triggered, the request is sent to `assistant.model`.
    #[serde(
        rename = "transcriptionEndpointingPlan",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcription_endpointing_plan: Option<models::TranscriptionEndpointingPlan>,
}

impl StartSpeakingPlan {
    pub fn new() -> StartSpeakingPlan {
        StartSpeakingPlan {
            wait_seconds: None,
            smart_endpointing_enabled: None,
            custom_endpointing_rules: None,
            transcription_endpointing_plan: None,
        }
    }
}
