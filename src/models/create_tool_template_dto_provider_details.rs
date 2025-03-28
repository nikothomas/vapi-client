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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
#[serde(untagged)]
pub enum CreateToolTemplateDtoProviderDetails {
    MakeToolProviderDetails(models::MakeToolProviderDetails),
    GhlToolProviderDetails(models::GhlToolProviderDetails),
    FunctionToolProviderDetails(models::FunctionToolProviderDetails),
}

impl Default for CreateToolTemplateDtoProviderDetails {
    fn default() -> Self {
        Self::MakeToolProviderDetails(Default::default())
    }
}
/// The type of tool. \"make\" for Make tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "ghl")]
    Ghl,
    #[serde(rename = "function")]
    Function,
}

impl Default for Type {
    fn default() -> Type {
        Self::Make
    }
}
