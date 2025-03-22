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
pub struct ServerMessage {
    #[serde(rename = "message")]
    pub message: models::ServerMessageMessage,
}

impl ServerMessage {
    pub fn new(message: models::ServerMessageMessage) -> ServerMessage {
        ServerMessage { message }
    }
}
