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
pub struct UpdateTokenDto {
    /// This is the tag for the token. It represents its scope.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
    /// This is the name of the token. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This are the restrictions for the token.
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<models::TokenRestrictions>,
}

impl UpdateTokenDto {
    pub fn new() -> UpdateTokenDto {
        UpdateTokenDto {
            tag: None,
            name: None,
            restrictions: None,
        }
    }
}
/// This is the tag for the token. It represents its scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Tag {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}

impl Default for Tag {
    fn default() -> Tag {
        Self::Private
    }
}
