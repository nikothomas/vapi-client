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
pub struct SupabaseBucketPlan {
    /// This is the S3 Region. It should look like us-east-1 It should be one of the supabase regions defined in the SUPABASE_REGION enum Check https://supabase.com/docs/guides/platform/regions for up to date regions
    #[serde(rename = "region")]
    pub region: RegionTrue,
    /// This is the S3 compatible URL for Supabase S3 This should look like https://<project-ID>.supabase.co/storage/v1/s3
    #[serde(rename = "url")]
    pub url: String,
    /// This is the Supabase S3 Access Key ID. The user creates this in the Supabase project Storage settings
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
    /// This is the Supabase S3 Secret Access Key. The user creates this in the Supabase project Storage settings along with the access key id
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
    /// This is the Supabase S3 Bucket Name. The user must create this in Supabase under Storage > Buckets A bucket that does not exist will not be checked now, but file uploads will fail
    #[serde(rename = "name")]
    pub name: String,
    /// This is the Supabase S3 Bucket Folder Path. The user can create this in Supabase under Storage > Buckets A path that does not exist will not be checked now, but file uploads will fail A Path is like a folder in the bucket Eg. If the bucket is called \"my-bucket\" and the path is \"my-folder\", the full path is \"my-bucket/my-folder\"
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl SupabaseBucketPlan {
    pub fn new(
        region: RegionTrue,
        url: String,
        access_key_id: String,
        secret_access_key: String,
        name: String,
    ) -> SupabaseBucketPlan {
        SupabaseBucketPlan {
            region,
            url,
            access_key_id,
            secret_access_key,
            name,
            path: None,
        }
    }
}
/// This is the S3 Region. It should look like us-east-1 It should be one of the supabase regions defined in the SUPABASE_REGION enum Check https://supabase.com/docs/guides/platform/regions for up to date regions
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegionTrue {
    #[serde(rename = "us-west-1")]
    UsWest1,
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "us-east-2")]
    UsEast2,
    #[serde(rename = "ca-central-1")]
    CaCentral1,
    #[serde(rename = "eu-west-1")]
    EuWest1,
    #[serde(rename = "eu-west-2")]
    EuWest2,
    #[serde(rename = "eu-west-3")]
    EuWest3,
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    #[serde(rename = "eu-central-2")]
    EuCentral2,
    #[serde(rename = "eu-north-1")]
    EuNorth1,
    #[serde(rename = "ap-south-1")]
    ApSouth1,
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,
    #[serde(rename = "ap-northeast-2")]
    ApNortheast2,
    #[serde(rename = "ap-southeast-2")]
    ApSoutheast2,
    #[serde(rename = "sa-east-1")]
    SaEast1,
}

impl Default for RegionTrue {
    fn default() -> RegionTrue {
        Self::UsWest1
    }
}
