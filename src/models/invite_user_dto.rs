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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct InviteUserDto {
    #[serde(rename = "emails")]
    pub emails: Vec<String>,
    #[serde(rename = "role")]
    pub role: Role,
    #[serde(rename = "redirectTo", skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<String>,
}

impl InviteUserDto {
    pub fn new(emails: Vec<String>, role: Role) -> InviteUserDto {
        InviteUserDto {
            emails,
            role,
            redirect_to: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "viewer")]
    Viewer,
}

impl Default for Role {
    fn default() -> Role {
        Self::Admin
    }
}
