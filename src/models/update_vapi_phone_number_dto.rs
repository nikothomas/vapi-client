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
pub struct UpdateVapiPhoneNumberDto {
    #[serde(rename = "fallbackDestination", skip_serializing_if = "Option::is_none")]
    pub fallback_destination: Option<Box<models::ImportTwilioPhoneNumberDtoFallbackDestination>>,
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
    /// This is the SIP URI of the phone number. You can SIP INVITE this. The assistant attached to this number will answer.  This is case-insensitive.
    #[serde(rename = "sipUri", skip_serializing_if = "Option::is_none")]
    pub sip_uri: Option<String>,
    /// This enables authentication for incoming SIP INVITE requests to the `sipUri`.  If not set, any username/password to the 401 challenge of the SIP INVITE will be accepted.
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Box<models::SipAuthentication>>,
}

impl UpdateVapiPhoneNumberDto {
    pub fn new() -> UpdateVapiPhoneNumberDto {
        UpdateVapiPhoneNumberDto {
            fallback_destination: None,
            name: None,
            assistant_id: None,
            squad_id: None,
            server: None,
            sip_uri: None,
            authentication: None,
        }
    }
}

