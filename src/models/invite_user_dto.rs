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
pub struct InviteUserDto {
    #[serde(rename = "emails")]
    pub emails: Vec<String>,
    #[serde(rename = "role")]
    pub role: RoleTrue,
    #[serde(rename = "redirectTo", skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<String>,
}

impl InviteUserDto {
    pub fn new(emails: Vec<String>, role: RoleTrue) -> InviteUserDto {
        InviteUserDto {
            emails,
            role,
            redirect_to: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoleTrue {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "viewer")]
    Viewer,
}

impl Default for RoleTrue {
    fn default() -> RoleTrue {
        Self::Admin
    }
}
