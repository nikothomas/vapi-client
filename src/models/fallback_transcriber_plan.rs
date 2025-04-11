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
pub struct FallbackTranscriberPlan {
    #[serde(rename = "transcribers")]
    pub transcribers: Vec<models::FallbackTranscriberPlanTranscribersItem>,
}

impl FallbackTranscriberPlan {
    pub fn new(transcribers: Vec<models::FallbackTranscriberPlanTranscribersItem>) -> FallbackTranscriberPlan {
        FallbackTranscriberPlan {
            transcribers,
        }
    }
}

