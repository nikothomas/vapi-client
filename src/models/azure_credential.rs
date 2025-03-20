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
pub struct AzureCredential {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the service being used in Azure.
    #[serde(rename = "service")]
    pub service: Service,
    /// This is the region of the Azure resource.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    /// This is not returned in the API.
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
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
    /// This is the bucket plan that can be provided to store call artifacts in Azure Blob Storage.
    #[serde(rename = "bucketPlan", skip_serializing_if = "Option::is_none")]
    pub bucket_plan: Option<models::AzureBlobStorageBucketPlan>,
}

impl AzureCredential {
    pub fn new(
        provider: Provider,
        service: Service,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
    ) -> AzureCredential {
        AzureCredential {
            provider,
            service,
            region: None,
            api_key: None,
            id,
            org_id,
            created_at,
            updated_at,
            name: None,
            bucket_plan: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "azure")]
    Azure,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Azure
    }
}
/// This is the service being used in Azure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Service {
    #[serde(rename = "speech")]
    Speech,
    #[serde(rename = "blob_storage")]
    BlobStorage,
}

impl Default for Service {
    fn default() -> Service {
        Self::Speech
    }
}
/// This is the region of the Azure resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "australia")]
    Australia,
    #[serde(rename = "canadaeast")]
    Canadaeast,
    #[serde(rename = "canadacentral")]
    Canadacentral,
    #[serde(rename = "eastus2")]
    Eastus2,
    #[serde(rename = "eastus")]
    Eastus,
    #[serde(rename = "france")]
    France,
    #[serde(rename = "india")]
    India,
    #[serde(rename = "japaneast")]
    Japaneast,
    #[serde(rename = "japanwest")]
    Japanwest,
    #[serde(rename = "uaenorth")]
    Uaenorth,
    #[serde(rename = "northcentralus")]
    Northcentralus,
    #[serde(rename = "norway")]
    Norway,
    #[serde(rename = "southcentralus")]
    Southcentralus,
    #[serde(rename = "swedencentral")]
    Swedencentral,
    #[serde(rename = "switzerland")]
    Switzerland,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "westus")]
    Westus,
    #[serde(rename = "westus3")]
    Westus3,
}

impl Default for Region {
    fn default() -> Region {
        Self::Australia
    }
}
