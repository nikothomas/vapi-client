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
pub struct Template {
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<models::CreateToolTemplateDtoDetails>,
    #[serde(rename = "providerDetails", skip_serializing_if = "Option::is_none")]
    pub provider_details: Option<models::CreateToolTemplateDtoProviderDetails>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<models::ToolTemplateMetadata>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The name of the template. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// The unique identifier for the template.
    #[serde(rename = "id")]
    pub id: String,
    /// The unique identifier for the organization that this template belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// The ISO 8601 date-time string of when the template was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The ISO 8601 date-time string of when the template was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Template {
    pub fn new(
        r#type: Type,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
    ) -> Template {
        Template {
            details: None,
            provider_details: None,
            metadata: None,
            visibility: None,
            r#type,
            name: None,
            provider: None,
            id,
            org_id,
            created_at,
            updated_at,
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
