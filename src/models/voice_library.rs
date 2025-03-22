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
pub struct VoiceLibrary {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// The ID of the voice provided by the provider.
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    /// The unique slug of the voice.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// The name of the voice.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The language of the voice.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The language code of the voice.
    #[serde(rename = "languageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// The model of the voice.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The supported models of the voice.
    #[serde(rename = "supportedModels", skip_serializing_if = "Option::is_none")]
    pub supported_models: Option<String>,
    /// The gender of the voice.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// The accent of the voice.
    #[serde(rename = "accent", skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// The preview URL of the voice.
    #[serde(rename = "previewUrl", skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    /// The description of the voice.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The credential ID of the voice.
    #[serde(rename = "credentialId", skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    /// The unique identifier for the voice library.
    #[serde(rename = "id")]
    pub id: String,
    /// The unique identifier for the organization that this voice library belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// The Public voice is shared accross all the organizations.
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    /// The deletion status of the voice.
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    /// The ISO 8601 date-time string of when the voice library was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The ISO 8601 date-time string of when the voice library was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl VoiceLibrary {
    pub fn new(
        id: String,
        org_id: String,
        is_public: bool,
        is_deleted: bool,
        created_at: String,
        updated_at: String,
    ) -> VoiceLibrary {
        VoiceLibrary {
            provider: None,
            provider_id: None,
            slug: None,
            name: None,
            language: None,
            language_code: None,
            model: None,
            supported_models: None,
            gender: None,
            accent: None,
            preview_url: None,
            description: None,
            credential_id: None,
            id,
            org_id,
            is_public,
            is_deleted,
            created_at,
            updated_at,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Provider {
    #[serde(rename = "vapi")]
    Vapi,
    #[serde(rename = "11labs")]
    Variant11labs,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "cartesia")]
    Cartesia,
    #[serde(rename = "custom-voice")]
    CustomVoice,
    #[serde(rename = "deepgram")]
    Deepgram,
    #[serde(rename = "hume")]
    Hume,
    #[serde(rename = "lmnt")]
    Lmnt,
    #[serde(rename = "neets")]
    Neets,
    #[serde(rename = "neuphonic")]
    Neuphonic,
    #[serde(rename = "openai")]
    Openai,
    #[serde(rename = "playht")]
    Playht,
    #[serde(rename = "rime-ai")]
    RimeAi,
    #[serde(rename = "smallest-ai")]
    SmallestAi,
    #[serde(rename = "tavus")]
    Tavus,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Vapi
    }
}
/// The gender of the voice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum Gender {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
}

impl Default for Gender {
    fn default() -> Gender {
        Self::Male
    }
}
