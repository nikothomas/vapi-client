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
pub struct PhoneNumbersListResponseItemOneOf4 {
    #[serde(rename = "fallbackDestination", skip_serializing_if = "Option::is_none")]
    pub fallback_destination: Option<models::TelnyxPhoneNumberFallbackDestination>,
    /// This is the unique identifier for the phone number.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the org that this phone number belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the phone number was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the phone number was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::TelnyxPhoneNumberStatus>,
    /// This is the name of the phone number. This is just for your own reference.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected.
    #[serde(rename = "assistantId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<Option<String>>,
    /// This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected.
    #[serde(rename = "squadId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub squad_id: Option<Option<String>>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
    /// These are the digits of the phone number you own on your Telnyx.
    #[serde(rename = "number")]
    pub number: String,
    /// This is the credential you added in dashboard.vapi.ai/keys. This is used to configure the number to send inbound calls to Vapi, make outbound calls and do live call updates like transfers and hangups.
    #[serde(rename = "credentialId")]
    pub credential_id: String,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl PhoneNumbersListResponseItemOneOf4 {
    pub fn new(id: String, org_id: String, created_at: String, updated_at: String, number: String, credential_id: String, provider: Provider) -> PhoneNumbersListResponseItemOneOf4 {
        PhoneNumbersListResponseItemOneOf4 {
            fallback_destination: None,
            id,
            org_id,
            created_at,
            updated_at,
            status: None,
            name: None,
            assistant_id: None,
            squad_id: None,
            server: None,
            number,
            credential_id,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "telnyx")]
    Telnyx,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Telnyx
    }
}

