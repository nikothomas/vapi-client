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
pub struct SquadMemberDto {
    /// This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead.
    #[serde(
        rename = "assistantId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assistant_id: Option<Option<String>>,
    /// This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead.
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    /// This can be used to override the assistant's settings and provide values for it's template variables.
    #[serde(rename = "assistantOverrides", skip_serializing_if = "Option::is_none")]
    pub assistant_overrides: Option<models::AssistantOverrides>,
    /// These are the others assistants that this assistant can transfer to.  If the assistant already has transfer call tool, these destinations are just appended to existing ones.
    #[serde(
        rename = "assistantDestinations",
        skip_serializing_if = "Option::is_none"
    )]
    pub assistant_destinations: Option<Vec<models::TransferDestinationAssistant>>,
}

impl SquadMemberDto {
    pub fn new() -> SquadMemberDto {
        SquadMemberDto {
            assistant_id: None,
            assistant: None,
            assistant_overrides: None,
            assistant_destinations: None,
        }
    }
}
