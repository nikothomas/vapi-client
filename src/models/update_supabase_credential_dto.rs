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
pub struct UpdateSupabaseCredentialDto {
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "bucketPlan", skip_serializing_if = "Option::is_none")]
    pub bucket_plan: Option<Box<models::SupabaseBucketPlan>>,
}

impl UpdateSupabaseCredentialDto {
    pub fn new() -> UpdateSupabaseCredentialDto {
        UpdateSupabaseCredentialDto {
            name: None,
            bucket_plan: None,
        }
    }
}

