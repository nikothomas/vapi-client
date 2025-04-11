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
pub struct CreateSupabaseCredentialDto {
    #[serde(rename = "bucketPlan", skip_serializing_if = "Option::is_none")]
    pub bucket_plan: Option<models::SupabaseBucketPlan>,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
}

impl CreateSupabaseCredentialDto {
    pub fn new() -> CreateSupabaseCredentialDto {
        CreateSupabaseCredentialDto {
            bucket_plan: None,
            name: None,
        }
    }
}

