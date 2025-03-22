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
pub struct Token {
    /// This is the tag for the token. It represents its scope.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
    /// This is the unique identifier for the token.
    #[serde(rename = "id")]
    pub id: String,
    /// This is unique identifier for the org that this token belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the token was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the token was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the token key.
    #[serde(rename = "value")]
    pub value: String,
    /// This is the name of the token. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This are the restrictions for the token.
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<models::TokenRestrictions>,
}

impl Token {
    pub fn new(
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
        value: String,
    ) -> Token {
        Token {
            tag: None,
            id,
            org_id,
            created_at,
            updated_at,
            value,
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
