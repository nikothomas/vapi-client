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
pub struct CreateOrgDto {
    /// When this is enabled, no logs, recordings, or transcriptions will be stored. At the end of the call, you will still receive an end-of-call-report message to store on your server. Defaults to false. When HIPAA is enabled, only OpenAI/Custom LLM or Azure Providers will be available for LLM and Voice respectively. This is due to the compliance requirements of HIPAA. Other providers may not meet these requirements.
    #[serde(rename = "hipaaEnabled", skip_serializing_if = "Option::is_none")]
    pub hipaa_enabled: Option<bool>,
    /// This is the ID of the subscription the org belongs to.
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// This is the name of the org. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the channel of the org. There is the cluster the API traffic for the org will be directed.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /// This is the monthly billing limit for the org. To go beyond $1000/mo, please contact us at support@vapi.ai.
    #[serde(rename = "billingLimit", skip_serializing_if = "Option::is_none")]
    pub billing_limit: Option<f64>,
    /// This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server 2. phoneNumber.server 3. org.server
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
    /// This is the concurrency limit for the org. This is the maximum number of calls that can be active at any given time. To go beyond 10, please contact us at support@vapi.ai.
    #[serde(rename = "concurrencyLimit", skip_serializing_if = "Option::is_none")]
    pub concurrency_limit: Option<f64>,
    /// Stores the information about the compliance plan enforced at the organization level. Currently pciEnabled is supported through this field. When this is enabled, any logs, recordings, or transcriptions will be shipped to the customer endpoints if provided else lost. At the end of the call, you will receive an end-of-call-report message to store on your server, if webhook is provided. Defaults to false. When PCI is enabled, only PCI-compliant Providers will be available for LLM, Voice and transcribers. This is due to the compliance requirements of PCI. Other providers may not meet these requirements.
    #[serde(rename = "compliancePlan", skip_serializing_if = "Option::is_none")]
    pub compliance_plan: Option<models::CompliancePlan>,
}

impl CreateOrgDto {
    pub fn new() -> CreateOrgDto {
        CreateOrgDto {
            hipaa_enabled: None,
            subscription_id: None,
            name: None,
            channel: None,
            billing_limit: None,
            server: None,
            concurrency_limit: None,
            compliance_plan: None,
        }
    }
}
/// This is the channel of the org. There is the cluster the API traffic for the org will be directed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "weekly")]
    Weekly,
}

impl Default for Channel {
    fn default() -> Channel {
        Self::Default
    }
}

