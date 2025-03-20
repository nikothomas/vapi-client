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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpCredential {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the unique identifier for the credential.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the org that this credential belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the credential was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the assistant was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the GCP key. This is the JSON that can be generated in the Google Cloud Console at https://console.cloud.google.com/iam-admin/serviceaccounts/details/<service-account-id>/keys.  The schema is identical to the JSON that GCP outputs.
    #[serde(rename = "gcpKey")]
    pub gcp_key: models::GcpKey,
    /// This is the bucket plan that can be provided to store call artifacts in GCP.
    #[serde(rename = "bucketPlan", skip_serializing_if = "Option::is_none")]
    pub bucket_plan: Option<models::BucketPlan>,
}

impl GcpCredential {
    pub fn new(
        provider: Provider,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
        gcp_key: models::GcpKey,
    ) -> GcpCredential {
        GcpCredential {
            provider,
            id,
            org_id,
            created_at,
            updated_at,
            name: None,
            gcp_key,
            bucket_plan: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "gcp")]
    Gcp,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Gcp
    }
}
