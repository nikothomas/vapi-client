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
pub struct ApiRequest {
    #[serde(rename = "method")]
    pub method: models::ApiRequestMethod,
    /// Api endpoint to send requests to.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<models::JsonSchema>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<models::JsonSchema>,
    #[serde(rename = "mode")]
    pub mode: models::ApiRequestMode,
    /// This is a list of hooks for a task. Each hook is a list of tasks to run on a trigger (such as on start, on failure, etc). Only Say is supported for now.
    #[serde(rename = "hooks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Option<Vec<models::Hook>>>,
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<models::JsonSchema>,
    #[serde(rename = "name")]
    pub name: String,
    /// This is for metadata you want to store on the task.
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
}

impl ApiRequest {
    pub fn new(method: models::ApiRequestMethod, url: String, mode: models::ApiRequestMode, name: String) -> ApiRequest {
        ApiRequest {
            method,
            url,
            headers: None,
            body: None,
            mode,
            hooks: None,
            output: None,
            name,
            metadata: None,
        }
    }
}

