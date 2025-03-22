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
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct BucketPlan {
    /// This is the name of the bucket.
    #[serde(rename = "name")]
    pub name: String,
    /// This is the region of the bucket.  Usage: - If `credential.type` is `aws`, then this is required. - If `credential.type` is `gcp`, then this is optional since GCP allows buckets to be accessed without a region but region is required for data residency requirements. Read here: https://cloud.google.com/storage/docs/request-endpoints
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// This is the path where call artifacts will be stored.  Usage: - To store call artifacts in a specific folder, set this to the full path. Eg. \"/folder-name1/folder-name2\". - To store call artifacts in the root of the bucket, leave this blank.  @default \"/\"
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// This is the HMAC access key offered by GCP for interoperability with S3 clients. Here is the guide on how to create: https://cloud.google.com/storage/docs/authentication/managing-hmackeys#console  Usage: - If `credential.type` is `gcp`, then this is required. - If `credential.type` is `aws`, then this is not required since credential.awsAccessKeyId is used instead.
    #[serde(rename = "hmacAccessKey", skip_serializing_if = "Option::is_none")]
    pub hmac_access_key: Option<String>,
    /// This is the secret for the HMAC access key. Here is the guide on how to create: https://cloud.google.com/storage/docs/authentication/managing-hmackeys#console  Usage: - If `credential.type` is `gcp`, then this is required. - If `credential.type` is `aws`, then this is not required since credential.awsSecretAccessKey is used instead.  Note: This is not returned in the API.
    #[serde(rename = "hmacSecret", skip_serializing_if = "Option::is_none")]
    pub hmac_secret: Option<String>,
}

impl BucketPlan {
    pub fn new(name: String) -> BucketPlan {
        BucketPlan {
            name,
            region: None,
            path: None,
            hmac_access_key: None,
            hmac_secret: None,
        }
    }
}
