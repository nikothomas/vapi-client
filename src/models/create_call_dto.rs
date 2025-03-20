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
pub struct CreateCallDto {
    /// This is the name of the call. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead.
    #[serde(rename = "assistantId", skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead.
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    /// These are the overrides for the `assistant` or `assistantId`'s settings and template variables.
    #[serde(rename = "assistantOverrides", skip_serializing_if = "Option::is_none")]
    pub assistant_overrides: Option<models::AssistantOverrides>,
    /// This is the squad that will be used for the call. To use a transient squad, use `squad` instead.
    #[serde(rename = "squadId", skip_serializing_if = "Option::is_none")]
    pub squad_id: Option<String>,
    /// This is a squad that will be used for the call. To use an existing squad, use `squadId` instead.
    #[serde(rename = "squad", skip_serializing_if = "Option::is_none")]
    pub squad: Option<models::CreateSquadDto>,
    /// This is the phone number that will be used for the call. To use a transient number, use `phoneNumber` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "phoneNumberId", skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    /// This is the phone number that will be used for the call. To use an existing number, use `phoneNumberId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ImportTwilioPhoneNumberDto>,
    /// This is the customer that will be called. To call a transient customer , use `customer` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// This is the customer that will be called. To call an existing customer, use `customerId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<models::CreateCustomerDto>,
}

impl CreateCallDto {
    pub fn new() -> CreateCallDto {
        CreateCallDto {
            name: None,
            assistant_id: None,
            assistant: None,
            assistant_overrides: None,
            squad_id: None,
            squad: None,
            phone_number_id: None,
            phone_number: None,
            customer_id: None,
            customer: None,
        }
    }
}
