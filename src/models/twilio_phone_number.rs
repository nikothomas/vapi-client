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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct TwilioPhoneNumber {
    #[serde(
        rename = "fallbackDestination",
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_destination: Option<models::ImportTwilioPhoneNumberDtoFallbackDestination>,
    /// This is to use numbers bought on Twilio.
    #[serde(rename = "provider")]
    pub provider: Provider,
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
    /// This is the status of the phone number.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
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
    pub server: Option<models::Server>,
    /// These are the digits of the phone number you own on your Twilio.
    #[serde(rename = "number")]
    pub number: String,
    /// This is the Twilio Account SID for the phone number.
    #[serde(rename = "twilioAccountSid")]
    pub twilio_account_sid: String,
    /// This is the Twilio Auth Token for the phone number.
    #[serde(rename = "twilioAuthToken")]
    pub twilio_auth_token: String,
}

impl TwilioPhoneNumber {
    pub fn new(
        provider: Provider,
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
        number: String,
        twilio_account_sid: String,
        twilio_auth_token: String,
    ) -> TwilioPhoneNumber {
        TwilioPhoneNumber {
            fallback_destination: None,
            provider,
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
            twilio_account_sid,
            twilio_auth_token,
        }
    }
}
/// This is to use numbers bought on Twilio.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}
/// This is the status of the phone number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "activating")]
    Activating,
    #[serde(rename = "blocked")]
    Blocked,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
