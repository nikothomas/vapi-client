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
pub struct CreateTelnyxPhoneNumberDto {
    #[serde(rename = "fallbackDestination", skip_serializing_if = "Option::is_none")]
    pub fallback_destination: Option<Box<models::ImportTwilioPhoneNumberDtoFallbackDestination>>,
    /// This is to use numbers bought on Telnyx.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// These are the digits of the phone number you own on your Telnyx.
    #[serde(rename = "number")]
    pub number: String,
    /// This is the credential you added in dashboard.vapi.ai/keys. This is used to configure the number to send inbound calls to Vapi, make outbound calls and do live call updates like transfers and hangups.
    #[serde(rename = "credentialId")]
    pub credential_id: String,
    /// This is the name of the phone number. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected.
    #[serde(rename = "assistantId", skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected.
    #[serde(rename = "squadId", skip_serializing_if = "Option::is_none")]
    pub squad_id: Option<String>,
    /// This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server 2. phoneNumber.server 3. org.server
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<models::Server>>,
}

impl CreateTelnyxPhoneNumberDto {
    pub fn new(provider: Provider, number: String, credential_id: String) -> CreateTelnyxPhoneNumberDto {
        CreateTelnyxPhoneNumberDto {
            fallback_destination: None,
            provider,
            number,
            credential_id,
            name: None,
            assistant_id: None,
            squad_id: None,
            server: None,
        }
    }
}
/// This is to use numbers bought on Telnyx.
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

