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
pub struct VoiceCost {
    /// This is the type of cost, always 'voice' for this class.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the voice that was used during the call.  This matches one of the following: - `call.assistant.voice`, - `call.assistantId->voice`, - `call.squad[n].assistant.voice`, - `call.squad[n].assistantId->voice`, - `call.squadId->[n].assistant.voice`, - `call.squadId->[n].assistantId->voice`.
    #[serde(rename = "voice")]
    pub voice: serde_json::Value,
    /// This is the number of characters that were generated during the call. These should be total characters used in the call for single assistant calls, while squad calls will have multiple voice costs one for each assistant that was used.
    #[serde(rename = "characters")]
    pub characters: f64,
    /// This is the cost of the component in USD.
    #[serde(rename = "cost")]
    pub cost: f64,
}

impl VoiceCost {
    pub fn new(r#type: Type, voice: serde_json::Value, characters: f64, cost: f64) -> VoiceCost {
        VoiceCost {
            r#type,
            voice,
            characters,
            cost,
        }
    }
}
/// This is the type of cost, always 'voice' for this class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Type {
    #[serde(rename = "voice")]
    Voice,
}

impl Default for Type {
    fn default() -> Type {
        Self::Voice
    }
}
