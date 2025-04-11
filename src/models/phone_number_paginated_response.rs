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
pub struct PhoneNumberPaginatedResponse {
    /// A list of phone numbers, which can be of any provider type.
    #[serde(rename = "results")]
    pub results: Vec<models::PhoneNumberPaginatedResponseResultsItem>,
    #[serde(rename = "metadata")]
    pub metadata: models::PaginationMeta,
}

impl PhoneNumberPaginatedResponse {
    pub fn new(results: Vec<models::PhoneNumberPaginatedResponseResultsItem>, metadata: models::PaginationMeta) -> PhoneNumberPaginatedResponse {
        PhoneNumberPaginatedResponse {
            results,
            metadata,
        }
    }
}

