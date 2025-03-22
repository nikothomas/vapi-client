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
pub struct Squad {
    /// This is the name of the squad.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the list of assistants that make up the squad.  The call will start with the first assistant in the list.
    #[serde(rename = "members")]
    pub members: Vec<models::SquadMemberDto>,
    /// This can be used to override all the assistants' settings and provide values for their template variables.  Both `membersOverrides` and `members[n].assistantOverrides` can be used together. First, `members[n].assistantOverrides` is applied. Then, `membersOverrides` is applied as a global override.
    #[serde(rename = "membersOverrides", skip_serializing_if = "Option::is_none")]
    pub members_overrides: Option<models::AssistantOverrides>,
    /// This is the unique identifier for the squad.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the org that this squad belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the squad was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the squad was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Squad {
    pub fn new(
        members: Vec<models::SquadMemberDto>,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
    ) -> Squad {
        Squad {
            name: None,
            members,
            members_overrides: None,
            id,
            org_id,
            created_at,
            updated_at,
        }
    }
}
