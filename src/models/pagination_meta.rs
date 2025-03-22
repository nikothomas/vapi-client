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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: f64,
    #[serde(rename = "totalItems")]
    pub total_items: f64,
    #[serde(rename = "currentPage")]
    pub current_page: f64,
}

impl PaginationMeta {
    pub fn new(items_per_page: f64, total_items: f64, current_page: f64) -> PaginationMeta {
        PaginationMeta {
            items_per_page,
            total_items,
            current_page,
        }
    }
}
