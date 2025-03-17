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
pub struct CartesiaExperimentalControls {
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Speed>,
    #[serde(rename = "emotion", skip_serializing_if = "Option::is_none")]
    pub emotion: Option<Emotion>,
}

impl CartesiaExperimentalControls {
    pub fn new() -> CartesiaExperimentalControls {
        CartesiaExperimentalControls {
            speed: None,
            emotion: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Speed {
    #[serde(rename = "slowest")]
    Slowest,
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "fast")]
    Fast,
    #[serde(rename = "fastest")]
    Fastest,
}

impl Default for Speed {
    fn default() -> Speed {
        Self::Slowest
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Emotion {
    #[serde(rename = "anger:lowest")]
    AngerColonLowest,
    #[serde(rename = "anger:low")]
    AngerColonLow,
    #[serde(rename = "anger:high")]
    AngerColonHigh,
    #[serde(rename = "anger:highest")]
    AngerColonHighest,
    #[serde(rename = "positivity:lowest")]
    PositivityColonLowest,
    #[serde(rename = "positivity:low")]
    PositivityColonLow,
    #[serde(rename = "positivity:high")]
    PositivityColonHigh,
    #[serde(rename = "positivity:highest")]
    PositivityColonHighest,
    #[serde(rename = "surprise:lowest")]
    SurpriseColonLowest,
    #[serde(rename = "surprise:low")]
    SurpriseColonLow,
    #[serde(rename = "surprise:high")]
    SurpriseColonHigh,
    #[serde(rename = "surprise:highest")]
    SurpriseColonHighest,
    #[serde(rename = "sadness:lowest")]
    SadnessColonLowest,
    #[serde(rename = "sadness:low")]
    SadnessColonLow,
    #[serde(rename = "sadness:high")]
    SadnessColonHigh,
    #[serde(rename = "sadness:highest")]
    SadnessColonHighest,
    #[serde(rename = "curiosity:lowest")]
    CuriosityColonLowest,
    #[serde(rename = "curiosity:low")]
    CuriosityColonLow,
    #[serde(rename = "curiosity:high")]
    CuriosityColonHigh,
    #[serde(rename = "curiosity:highest")]
    CuriosityColonHighest,
}

impl Default for Emotion {
    fn default() -> Emotion {
        Self::AngerColonLowest
    }
}

