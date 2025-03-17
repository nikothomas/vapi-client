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
pub struct UpdateToolTemplateDto {
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<models::CreateToolTemplateDtoDetails>>,
    #[serde(rename = "providerDetails", skip_serializing_if = "Option::is_none")]
    pub provider_details: Option<Box<models::CreateToolTemplateDtoProviderDetails>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::ToolTemplateMetadata>>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The name of the template. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
}

impl UpdateToolTemplateDto {
    pub fn new(r#type: Type) -> UpdateToolTemplateDto {
        UpdateToolTemplateDto {
            details: None,
            provider_details: None,
            metadata: None,
            visibility: None,
            r#type,
            name: None,
            provider: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "tool")]
    Tool,
}

impl Default for Type {
    fn default() -> Type {
        Self::Tool
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "gohighlevel")]
    Gohighlevel,
    #[serde(rename = "function")]
    Function,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Make
    }
}

