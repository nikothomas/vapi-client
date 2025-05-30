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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowNodesInner {
    Say(models::Say),
    Gather(models::Gather),
    ApiRequest(models::ApiRequest),
    Hangup(models::Hangup),
    Transfer(models::Transfer),
}

impl Default for WorkflowNodesInner {
    fn default() -> Self {
        Self::Say(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "say")]
    Say,
    #[serde(rename = "gather")]
    Gather,
    #[serde(rename = "apiRequest")]
    ApiRequest,
    #[serde(rename = "hangup")]
    Hangup,
    #[serde(rename = "transfer")]
    Transfer,
}

impl Default for Type {
    fn default() -> Type {
        Self::Say
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "GET")]
    Get,
}

impl Default for Method {
    fn default() -> Method {
        Self::Post
    }
}
/// This is the mode of the Api Request. We only support BLOCKING and BACKGROUND for now.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "blocking")]
    Blocking,
    #[serde(rename = "background")]
    Background,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Blocking
    }
}

