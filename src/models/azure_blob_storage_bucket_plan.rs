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
pub struct AzureBlobStorageBucketPlan {
    /// This is the blob storage connection string for the Azure resource.
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    /// This is the container name for the Azure blob storage.
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// This is the path where call artifacts will be stored.  Usage: - To store call artifacts in a specific folder, set this to the full path. Eg. \"/folder-name1/folder-name2\". - To store call artifacts in the root of the bucket, leave this blank.  @default \"/\"
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
}

impl AzureBlobStorageBucketPlan {
    pub fn new(connection_string: String, container_name: String) -> AzureBlobStorageBucketPlan {
        AzureBlobStorageBucketPlan {
            connection_string,
            container_name,
            path: None,
        }
    }
}

