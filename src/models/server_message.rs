/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerMessage {
    #[serde(rename = "message")]
    pub message: Box<models::ServerMessageMessage>,
}

impl ServerMessage {
    pub fn new(message: models::ServerMessageMessage) -> ServerMessage {
        ServerMessage {
            message: Box::new(message),
        }
    }
}

