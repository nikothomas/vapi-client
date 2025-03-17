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
pub struct Org {
    /// When this is enabled, no logs, recordings, or transcriptions will be stored. At the end of the call, you will still receive an end-of-call-report message to store on your server. Defaults to false. When HIPAA is enabled, only OpenAI/Custom LLM or Azure Providers will be available for LLM and Voice respectively. This is due to the compliance requirements of HIPAA. Other providers may not meet these requirements.
    #[serde(rename = "hipaaEnabled", skip_serializing_if = "Option::is_none")]
    pub hipaa_enabled: Option<bool>,
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<models::Subscription>>,
    /// This is the ID of the subscription the org belongs to.
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// This is the unique identifier for the org.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the ISO 8601 date-time string of when the org was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the org was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the Stripe customer for the org.
    #[serde(rename = "stripeCustomerId", skip_serializing_if = "Option::is_none")]
    pub stripe_customer_id: Option<String>,
    /// This is the subscription for the org.
    #[serde(rename = "stripeSubscriptionId", skip_serializing_if = "Option::is_none")]
    pub stripe_subscription_id: Option<String>,
    /// This is the subscription's subscription item.
    #[serde(rename = "stripeSubscriptionItemId", skip_serializing_if = "Option::is_none")]
    pub stripe_subscription_item_id: Option<String>,
    /// This is the subscription's current period start.
    #[serde(rename = "stripeSubscriptionCurrentPeriodStart", skip_serializing_if = "Option::is_none")]
    pub stripe_subscription_current_period_start: Option<String>,
    /// This is the subscription's status.
    #[serde(rename = "stripeSubscriptionStatus", skip_serializing_if = "Option::is_none")]
    pub stripe_subscription_status: Option<String>,
    /// This is the plan for the org.
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<models::OrgPlan>>,
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
    pub server: Option<Box<models::Server>>,
    /// This is the concurrency limit for the org. This is the maximum number of calls that can be active at any given time. To go beyond 10, please contact us at support@vapi.ai.
    #[serde(rename = "concurrencyLimit", skip_serializing_if = "Option::is_none")]
    pub concurrency_limit: Option<f64>,
}

impl Org {
    pub fn new(id: String, created_at: String, updated_at: String) -> Org {
        Org {
            hipaa_enabled: None,
            subscription: None,
            subscription_id: None,
            id,
            created_at,
            updated_at,
            stripe_customer_id: None,
            stripe_subscription_id: None,
            stripe_subscription_item_id: None,
            stripe_subscription_current_period_start: None,
            stripe_subscription_status: None,
            plan: None,
            name: None,
            channel: None,
            billing_limit: None,
            server: None,
            concurrency_limit: None,
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

