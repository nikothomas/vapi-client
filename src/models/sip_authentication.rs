/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SipAuthentication {
    /// This will be expected in the `realm` field of the `authorization` header of the SIP INVITE. Defaults to sip.vapi.ai.
    #[serde(rename = "realm", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub realm: Option<Option<String>>,
    /// This will be expected in the `username` field of the `authorization` header of the SIP INVITE.
    #[serde(rename = "username")]
    pub username: String,
    /// This will be expected to generate the `response` field of the `authorization` header of the SIP INVITE, through digest authentication.
    #[serde(rename = "password")]
    pub password: String,
}

impl SipAuthentication {
    pub fn new(username: String, password: String) -> SipAuthentication {
        SipAuthentication {
            realm: None,
            username,
            password,
        }
    }
}

