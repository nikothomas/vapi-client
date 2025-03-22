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
use utoipa::OpenApi;


use crate::models;

/// ToolCallBlockTool : This is the tool that the block will call. To use an existing tool, use `toolId`.
/// This is the tool that the block will call. To use an existing tool, use `toolId`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(untagged)]
pub enum ToolCallBlockTool {
    CreateDtmfToolDto(models::CreateDtmfToolDto),
    CreateEndCallToolDto(models::CreateEndCallToolDto),
    CreateVoicemailToolDto(models::CreateVoicemailToolDto),
    CreateFunctionToolDto(models::CreateFunctionToolDto),
    CreateGhlToolDto(models::CreateGhlToolDto),
    CreateMakeToolDto(models::CreateMakeToolDto),
    CreateTransferCallToolDto(models::CreateTransferCallToolDto),
}

impl Default for ToolCallBlockTool {
    fn default() -> Self {
        Self::CreateDtmfToolDto(Default::default())
    }
}
/// The type of tool. \"dtmf\" for DTMF tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
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
