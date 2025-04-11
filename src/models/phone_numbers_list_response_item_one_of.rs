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
pub struct PhoneNumbersListResponseItemOneOf {
    #[serde(rename = "fallbackDestination", skip_serializing_if = "Option::is_none")]
    pub fallback_destination: Option<models::ByoPhoneNumberFallbackDestination>,
    /// This is the flag to toggle the E164 check for the `number` field. This is an advanced property which should be used if you know your use case requires it.  Use cases: - `false`: To allow non-E164 numbers like `+001234567890`, `1234`, or `abc`. This is useful for dialing out to non-E164 numbers on your SIP trunks. - `true` (default): To allow only E164 numbers like `+14155551234`. This is standard for PSTN calls.  If `false`, the `number` is still required to only contain alphanumeric characters (regex: `/^\\+?[a-zA-Z0-9]+$/`).  @default true (E164 check is enabled)
    #[serde(rename = "numberE164CheckEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number_e164_check_enabled: Option<Option<bool>>,
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
    pub status: Option<models::ByoPhoneNumberStatus>,
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
    /// This is the number of the customer.
    #[serde(rename = "number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number: Option<Option<String>>,
    /// This is the credential of your own SIP trunk or Carrier (type `byo-sip-trunk`) which can be used to make calls to this phone number.  You can add the SIP trunk or Carrier credential in the Provider Credentials page on the Dashboard to get the credentialId.
    #[serde(rename = "credentialId")]
    pub credential_id: String,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl PhoneNumbersListResponseItemOneOf {
    pub fn new(id: String, org_id: String, created_at: String, updated_at: String, credential_id: String, provider: Provider) -> PhoneNumbersListResponseItemOneOf {
        PhoneNumbersListResponseItemOneOf {
            fallback_destination: None,
            number_e164_check_enabled: None,
            id,
            org_id,
            created_at,
            updated_at,
            status: None,
            name: None,
            assistant_id: None,
            squad_id: None,
            server: None,
            number: None,
            credential_id,
            provider,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "byo-phone-number")]
    ByoPhoneNumber,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::ByoPhoneNumber
    }
}

