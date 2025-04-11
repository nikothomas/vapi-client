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
pub struct ServerMessageResponse {
    #[serde(rename = "messageResponse")]
    pub message_response: models::ServerMessageResponseMessageResponse,
}

impl ServerMessageResponse {
    pub fn new(message_response: models::ServerMessageResponseMessageResponse) -> ServerMessageResponse {
        ServerMessageResponse {
            message_response,
        }
    }
}

