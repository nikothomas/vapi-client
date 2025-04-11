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
pub struct SyncVoiceLibraryDto {
    /// List of providers you want to sync.
    #[serde(rename = "providers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Option<Vec<models::SyncVoiceLibraryDtoProvidersItem>>>,
}

impl SyncVoiceLibraryDto {
    pub fn new() -> SyncVoiceLibraryDto {
        SyncVoiceLibraryDto {
            providers: None,
        }
    }
}

