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
pub struct CreateVapiPhoneNumberDto {
    #[serde(rename = "fallbackDestination", skip_serializing_if = "Option::is_none")]
    pub fallback_destination: Option<models::CreateVapiPhoneNumberDtoFallbackDestination>,
    /// This is the area code of the phone number to purchase.
    #[serde(rename = "numberDesiredAreaCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number_desired_area_code: Option<Option<String>>,
    /// This is the SIP URI of the phone number. You can SIP INVITE this. The assistant attached to this number will answer.  This is case-insensitive.
    #[serde(rename = "sipUri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sip_uri: Option<Option<String>>,
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<models::SipAuthentication>,
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
}

impl CreateVapiPhoneNumberDto {
    pub fn new() -> CreateVapiPhoneNumberDto {
        CreateVapiPhoneNumberDto {
            fallback_destination: None,
            number_desired_area_code: None,
            sip_uri: None,
            authentication: None,
            name: None,
            assistant_id: None,
            squad_id: None,
            server: None,
        }
    }
}

