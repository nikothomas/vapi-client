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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientMessage {
    #[serde(rename = "message")]
    pub message: models::ClientMessageMessage,
}

impl ClientMessage {
    pub fn new(message: models::ClientMessageMessage) -> ClientMessage {
        ClientMessage { message }
    }
}
