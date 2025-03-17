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
pub struct UpdateCloudflareCredentialDto {
    /// Cloudflare Account Id.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Cloudflare API Key / Token.
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Cloudflare Account Email.
    #[serde(rename = "accountEmail", skip_serializing_if = "Option::is_none")]
    pub account_email: Option<String>,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the bucket plan that can be provided to store call artifacts in R2
    #[serde(rename = "bucketPlan", skip_serializing_if = "Option::is_none")]
    pub bucket_plan: Option<Box<models::CloudflareR2BucketPlan>>,
}

impl UpdateCloudflareCredentialDto {
    pub fn new() -> UpdateCloudflareCredentialDto {
        UpdateCloudflareCredentialDto {
            account_id: None,
            api_key: None,
            account_email: None,
            name: None,
            bucket_plan: None,
        }
    }
}

