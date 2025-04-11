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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BashToolWithToolCallMessagesItem {
    CreateDtmfToolDtoMessagesItemOneOf(models::CreateDtmfToolDtoMessagesItemOneOf),
    CreateDtmfToolDtoMessagesItemOneOf1(models::CreateDtmfToolDtoMessagesItemOneOf1),
    CreateDtmfToolDtoMessagesItemOneOf2(models::CreateDtmfToolDtoMessagesItemOneOf2),
    CreateDtmfToolDtoMessagesItemOneOf3(models::CreateDtmfToolDtoMessagesItemOneOf3),
}

impl Default for BashToolWithToolCallMessagesItem {
    fn default() -> Self {
        Self::CreateDtmfToolDtoMessagesItemOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "request-start")]
    RequestStart,
    #[serde(rename = "request-complete")]
    RequestComplete,
    #[serde(rename = "request-failed")]
    RequestFailed,
    #[serde(rename = "request-response-delayed")]
    RequestResponseDelayed,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequestStart
    }
}

