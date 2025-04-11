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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ElevenLabsVoiceIdEnum {
    #[serde(rename = "burt")]
    Burt,
    #[serde(rename = "marissa")]
    Marissa,
    #[serde(rename = "andrea")]
    Andrea,
    #[serde(rename = "sarah")]
    Sarah,
    #[serde(rename = "phillip")]
    Phillip,
    #[serde(rename = "steve")]
    Steve,
    #[serde(rename = "joseph")]
    Joseph,
    #[serde(rename = "myra")]
    Myra,
    #[serde(rename = "paula")]
    Paula,
    #[serde(rename = "ryan")]
    Ryan,
    #[serde(rename = "drew")]
    Drew,
    #[serde(rename = "paul")]
    Paul,
    #[serde(rename = "mrb")]
    Mrb,
    #[serde(rename = "matilda")]
    Matilda,
    #[serde(rename = "mark")]
    Mark,

}

impl std::fmt::Display for ElevenLabsVoiceIdEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Burt => write!(f, "burt"),
            Self::Marissa => write!(f, "marissa"),
            Self::Andrea => write!(f, "andrea"),
            Self::Sarah => write!(f, "sarah"),
            Self::Phillip => write!(f, "phillip"),
            Self::Steve => write!(f, "steve"),
            Self::Joseph => write!(f, "joseph"),
            Self::Myra => write!(f, "myra"),
            Self::Paula => write!(f, "paula"),
            Self::Ryan => write!(f, "ryan"),
            Self::Drew => write!(f, "drew"),
            Self::Paul => write!(f, "paul"),
            Self::Mrb => write!(f, "mrb"),
            Self::Matilda => write!(f, "matilda"),
            Self::Mark => write!(f, "mark"),
        }
    }
}

impl Default for ElevenLabsVoiceIdEnum {
    fn default() -> ElevenLabsVoiceIdEnum {
        Self::Burt
    }
}

