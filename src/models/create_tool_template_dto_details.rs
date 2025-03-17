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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateToolTemplateDtoDetails {
    CreateDtmfToolDto(Box<models::CreateDtmfToolDto>),
    CreateEndCallToolDto(Box<models::CreateEndCallToolDto>),
    CreateVoicemailToolDto(Box<models::CreateVoicemailToolDto>),
    CreateFunctionToolDto(Box<models::CreateFunctionToolDto>),
    CreateGhlToolDto(Box<models::CreateGhlToolDto>),
    CreateMakeToolDto(Box<models::CreateMakeToolDto>),
    CreateTransferCallToolDto(Box<models::CreateTransferCallToolDto>),
}

impl Default for CreateToolTemplateDtoDetails {
    fn default() -> Self {
        Self::CreateDtmfToolDto(Default::default())
    }
}
/// The type of tool. \"dtmf\" for DTMF tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dtmf")]
    Dtmf,
    #[serde(rename = "endCall")]
    EndCall,
    #[serde(rename = "voicemail")]
    Voicemail,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "ghl")]
    Ghl,
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "transferCall")]
    TransferCall,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dtmf
    }
}

