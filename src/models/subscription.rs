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
pub struct Subscription {
    /// This is the unique identifier for the subscription.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the timestamp when the subscription was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the timestamp when the subscription was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "type")]
    pub r#type: models::SubscriptionType,
    #[serde(rename = "status")]
    pub status: models::SubscriptionStatus,
    /// This is the number of credits the subscription currently has.  Note: This is a string to avoid floating point precision issues.
    #[serde(rename = "credits")]
    pub credits: String,
    /// This is the total number of active calls (concurrency) across all orgs under this subscription.
    #[serde(rename = "concurrencyCounter")]
    pub concurrency_counter: f64,
    /// This is the default concurrency limit for the subscription.
    #[serde(rename = "concurrencyLimitIncluded")]
    pub concurrency_limit_included: f64,
    /// This is the number of free phone numbers the subscription has
    #[serde(rename = "phoneNumbersCounter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_numbers_counter: Option<Option<f64>>,
    /// This is the maximum number of free phone numbers the subscription can have
    #[serde(rename = "phoneNumbersIncluded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_numbers_included: Option<Option<f64>>,
    /// This is the purchased add-on concurrency limit for the subscription.
    #[serde(rename = "concurrencyLimitPurchased")]
    pub concurrency_limit_purchased: f64,
    /// This is the ID of the monthly job that charges for subscription add ons and phone numbers.
    #[serde(rename = "monthlyChargeScheduleId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub monthly_charge_schedule_id: Option<Option<f64>>,
    /// This is the ID of the monthly job that checks whether the credit balance of the subscription is sufficient for the monthly charge.
    #[serde(rename = "monthlyCreditCheckScheduleId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub monthly_credit_check_schedule_id: Option<Option<f64>>,
    /// This is the Stripe customer ID.
    #[serde(rename = "stripeCustomerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stripe_customer_id: Option<Option<String>>,
    /// This is the Stripe payment ID.
    #[serde(rename = "stripePaymentMethodId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stripe_payment_method_id: Option<Option<String>>,
    /// If this flag is true, then the user has purchased slack support.
    #[serde(rename = "slackSupportEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slack_support_enabled: Option<Option<bool>>,
    /// If this subscription has a slack support subscription, the slack channel's ID will be stored here.
    #[serde(rename = "slackChannelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slack_channel_id: Option<Option<String>>,
    /// This is the HIPAA enabled flag for the subscription. It determines whether orgs under this subscription have the option to enable HIPAA compliance.
    #[serde(rename = "hipaaEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hipaa_enabled: Option<Option<bool>>,
    /// This is the ID for the Common Paper agreement outlining the HIPAA contract.
    #[serde(rename = "hipaaCommonPaperAgreementId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hipaa_common_paper_agreement_id: Option<Option<String>>,
    /// This is the Stripe fingerprint of the payment method (card). It allows us to detect users who try to abuse our system through multiple sign-ups.
    #[serde(rename = "stripePaymentMethodFingerprint", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stripe_payment_method_fingerprint: Option<Option<String>>,
    /// This is the customer's email on Stripe.
    #[serde(rename = "stripeCustomerEmail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stripe_customer_email: Option<Option<String>>,
    /// This is the email of the referrer for the subscription.
    #[serde(rename = "referredByEmail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub referred_by_email: Option<Option<String>>,
    #[serde(rename = "autoReloadPlan", skip_serializing_if = "Option::is_none")]
    pub auto_reload_plan: Option<models::AutoReloadPlan>,
    /// The number of minutes included in the subscription.
    #[serde(rename = "minutesIncluded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub minutes_included: Option<Option<f64>>,
    /// The number of minutes used in the subscription.
    #[serde(rename = "minutesUsed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub minutes_used: Option<Option<f64>>,
    /// This is the timestamp at which the number of monthly free minutes is scheduled to reset at.
    #[serde(rename = "minutesUsedNextResetAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub minutes_used_next_reset_at: Option<Option<String>>,
    /// The per minute charge on minutes that exceed the included minutes. Enterprise only.
    #[serde(rename = "minutesOverageCost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub minutes_overage_cost: Option<Option<f64>>,
    /// The list of providers included in the subscription. Enterprise only.
    #[serde(rename = "providersIncluded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub providers_included: Option<Option<Vec<String>>>,
    /// The maximum number of outbound calls this subscription may make in a day. Resets every night.
    #[serde(rename = "outboundCallsDailyLimit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outbound_calls_daily_limit: Option<Option<f64>>,
    /// The current number of outbound calls the subscription has made in the current day.
    #[serde(rename = "outboundCallsCounter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outbound_calls_counter: Option<Option<f64>>,
    /// This is the timestamp at which the outbound calls counter is scheduled to reset at.
    #[serde(rename = "outboundCallsCounterNextResetAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outbound_calls_counter_next_reset_at: Option<Option<String>>,
    /// This is the IDs of the coupons applicable to this subscription.
    #[serde(rename = "couponIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub coupon_ids: Option<Option<Vec<String>>>,
    /// This is the number of credits left obtained from a coupon.
    #[serde(rename = "couponUsageLeft", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub coupon_usage_left: Option<Option<String>>,
    #[serde(rename = "invoicePlan", skip_serializing_if = "Option::is_none")]
    pub invoice_plan: Option<models::InvoicePlan>,
    /// This is the PCI enabled flag for the subscription. It determines whether orgs under this subscription have the option to enable PCI compliance.
    #[serde(rename = "pciEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pci_enabled: Option<Option<bool>>,
    /// This is the ID for the Common Paper agreement outlining the PCI contract.
    #[serde(rename = "pciCommonPaperAgreementId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pci_common_paper_agreement_id: Option<Option<String>>,
}

impl Subscription {
    pub fn new(id: String, created_at: String, updated_at: String, r#type: models::SubscriptionType, status: models::SubscriptionStatus, credits: String, concurrency_counter: f64, concurrency_limit_included: f64, concurrency_limit_purchased: f64) -> Subscription {
        Subscription {
            id,
            created_at,
            updated_at,
            r#type,
            status,
            credits,
            concurrency_counter,
            concurrency_limit_included,
            phone_numbers_counter: None,
            phone_numbers_included: None,
            concurrency_limit_purchased,
            monthly_charge_schedule_id: None,
            monthly_credit_check_schedule_id: None,
            stripe_customer_id: None,
            stripe_payment_method_id: None,
            slack_support_enabled: None,
            slack_channel_id: None,
            hipaa_enabled: None,
            hipaa_common_paper_agreement_id: None,
            stripe_payment_method_fingerprint: None,
            stripe_customer_email: None,
            referred_by_email: None,
            auto_reload_plan: None,
            minutes_included: None,
            minutes_used: None,
            minutes_used_next_reset_at: None,
            minutes_overage_cost: None,
            providers_included: None,
            outbound_calls_daily_limit: None,
            outbound_calls_counter: None,
            outbound_calls_counter_next_reset_at: None,
            coupon_ids: None,
            coupon_usage_left: None,
            invoice_plan: None,
            pci_enabled: None,
            pci_common_paper_agreement_id: None,
        }
    }
}

