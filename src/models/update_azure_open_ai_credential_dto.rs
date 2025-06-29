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
pub struct UpdateAzureOpenAiCredentialDto {
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<RegionTrue>,
    #[serde(rename = "models", skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<ModelsTrue>>,
    /// This is not returned in the API.
    #[serde(rename = "openAIKey", skip_serializing_if = "Option::is_none")]
    pub open_ai_key: Option<String>,
    /// This is not returned in the API.
    #[serde(
        rename = "ocpApimSubscriptionKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub ocp_apim_subscription_key: Option<String>,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "openAIEndpoint", skip_serializing_if = "Option::is_none")]
    pub open_ai_endpoint: Option<String>,
}

impl UpdateAzureOpenAiCredentialDto {
    pub fn new() -> UpdateAzureOpenAiCredentialDto {
        UpdateAzureOpenAiCredentialDto {
            region: None,
            models: None,
            open_ai_key: None,
            ocp_apim_subscription_key: None,
            name: None,
            open_ai_endpoint: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegionTrue {
    #[serde(rename = "australia")]
    Australia,
    #[serde(rename = "canadaeast")]
    Canadaeast,
    #[serde(rename = "canadacentral")]
    Canadacentral,
    #[serde(rename = "eastus2")]
    Eastus2,
    #[serde(rename = "eastus")]
    Eastus,
    #[serde(rename = "france")]
    France,
    #[serde(rename = "india")]
    India,
    #[serde(rename = "japaneast")]
    Japaneast,
    #[serde(rename = "japanwest")]
    Japanwest,
    #[serde(rename = "uaenorth")]
    Uaenorth,
    #[serde(rename = "northcentralus")]
    Northcentralus,
    #[serde(rename = "norway")]
    Norway,
    #[serde(rename = "southcentralus")]
    Southcentralus,
    #[serde(rename = "swedencentral")]
    Swedencentral,
    #[serde(rename = "switzerland")]
    Switzerland,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "westus")]
    Westus,
    #[serde(rename = "westus3")]
    Westus3,
}

impl Default for RegionTrue {
    fn default() -> RegionTrue {
        Self::Australia
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ModelsTrue {
    #[serde(rename = "gpt-4.1-2025-04-14")]
    Gpt4Period120250414,
    #[serde(rename = "gpt-4.1-mini-2025-04-14")]
    Gpt4Period1Mini20250414,
    #[serde(rename = "gpt-4.1-nano-2025-04-14")]
    Gpt4Period1Nano20250414,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-35-turbo-0125")]
    Gpt35Turbo0125,
    #[serde(rename = "gpt-35-turbo-1106")]
    Gpt35Turbo1106,
}

impl Default for ModelsTrue {
    fn default() -> ModelsTrue {
        Self::Gpt4Period120250414
    }
}
