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
pub struct FallbackPlanVoicesItemOneOf3 {
    #[serde(rename = "server")]
    pub server: models::Server,
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl FallbackPlanVoicesItemOneOf3 {
    pub fn new(server: models::Server, provider: Provider) -> FallbackPlanVoicesItemOneOf3 {
        FallbackPlanVoicesItemOneOf3 {
            server,
            chunk_plan: None,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "custom-voice")]
    CustomVoice,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::CustomVoice
    }
}

