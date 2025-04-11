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
pub struct Hook {
    #[serde(rename = "on")]
    pub on: models::HookOn,
    #[serde(rename = "do")]
    pub r#do: Vec<models::SayHook>,
}

impl Hook {
    pub fn new(on: models::HookOn, r#do: Vec<models::SayHook>) -> Hook {
        Hook {
            on,
            r#do,
        }
    }
}

