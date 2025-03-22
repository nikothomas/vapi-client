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
pub struct ByoSipTrunkCredential {
    /// This can be used to bring your own SIP trunks or to connect to a Carrier.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// This is the unique identifier for the credential.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the org that this credential belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the credential was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the assistant was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the name of credential. This is just for your reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the list of SIP trunk's gateways.
    #[serde(rename = "gateways")]
    pub gateways: Vec<models::SipTrunkGateway>,
    /// This can be used to configure the outbound authentication if required by the SIP trunk.
    #[serde(
        rename = "outboundAuthenticationPlan",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_authentication_plan: Option<models::SipTrunkOutboundAuthenticationPlan>,
    /// This ensures the outbound origination attempts have a leading plus. Defaults to false to match conventional telecom behavior.  Usage: - Vonage/Twilio requires leading plus for all outbound calls. Set this to true.  @default false
    #[serde(
        rename = "outboundLeadingPlusEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_leading_plus_enabled: Option<bool>,
    /// This can be used to configure the tech prefix on outbound calls. This is an advanced property.
    #[serde(rename = "techPrefix", skip_serializing_if = "Option::is_none")]
    pub tech_prefix: Option<String>,
    /// This can be used to enable the SIP diversion header for authenticating the calling number if the SIP trunk supports it. This is an advanced property.
    #[serde(rename = "sipDiversionHeader", skip_serializing_if = "Option::is_none")]
    pub sip_diversion_header: Option<String>,
    /// This is an advanced configuration for enterprise deployments. This uses the onprem SBC to trunk into the SIP trunk's `gateways`, rather than the managed SBC provided by Vapi.
    #[serde(rename = "sbcConfiguration", skip_serializing_if = "Option::is_none")]
    pub sbc_configuration: Option<serde_json::Value>,
}

impl ByoSipTrunkCredential {
    pub fn new(
        id: String,
        org_id: String,
        created_at: String,
        updated_at: String,
        gateways: Vec<models::SipTrunkGateway>,
    ) -> ByoSipTrunkCredential {
        ByoSipTrunkCredential {
            provider: None,
            id,
            org_id,
            created_at,
            updated_at,
            name: None,
            gateways,
            outbound_authentication_plan: None,
            outbound_leading_plus_enabled: None,
            tech_prefix: None,
            sip_diversion_header: None,
            sbc_configuration: None,
        }
    }
}
/// This can be used to bring your own SIP trunks or to connect to a Carrier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "byo-sip-trunk")]
    ByoSipTrunk,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::ByoSipTrunk
    }
}
