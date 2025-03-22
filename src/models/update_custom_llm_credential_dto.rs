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
pub struct UpdateCustomLlmCredentialDto {
    /// This is not returned in the API.
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// This is the authentication plan. Currently supports OAuth2 RFC 6749. To use Bearer authentication, use apiKey
    #[serde(rename = "authenticationPlan", skip_serializing_if = "Option::is_none")]
    pub authentication_plan: Option<models::OAuth2AuthenticationPlan>,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateCustomLlmCredentialDto {
    pub fn new() -> UpdateCustomLlmCredentialDto {
        UpdateCustomLlmCredentialDto {
            api_key: None,
            authentication_plan: None,
            name: None,
        }
    }
}
