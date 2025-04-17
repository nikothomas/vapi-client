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
pub struct CreateSupabaseCredentialDto {
    /// This is for supabase storage.
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "bucketPlan", skip_serializing_if = "Option::is_none")]
    pub bucket_plan: Option<models::SupabaseBucketPlan>,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateSupabaseCredentialDto {
    pub fn new(provider: Provider) -> CreateSupabaseCredentialDto {
        CreateSupabaseCredentialDto {
            provider,
            bucket_plan: None,
            name: None,
        }
    }
}
/// This is for supabase storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "supabase")]
    Supabase,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Supabase
    }
}

