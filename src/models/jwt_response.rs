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
pub struct JwtResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "aud")]
    pub aud: serde_json::Value,
}

impl JwtResponse {
    pub fn new(access_token: String, aud: serde_json::Value) -> JwtResponse {
        JwtResponse { access_token, aud }
    }
}
