# Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | This is the unique identifier for the subscription. | 
**created_at** | **String** | This is the timestamp when the subscription was created. | 
**updated_at** | **String** | This is the timestamp when the subscription was last updated. | 
**r#type** | [**models::SubscriptionType**](SubscriptionType.md) |  | 
**status** | [**models::SubscriptionStatus**](SubscriptionStatus.md) |  | 
**credits** | **String** | This is the number of credits the subscription currently has.  Note: This is a string to avoid floating point precision issues. | 
**concurrency_counter** | **f64** | This is the total number of active calls (concurrency) across all orgs under this subscription. | 
**concurrency_limit_included** | **f64** | This is the default concurrency limit for the subscription. | 
**phone_numbers_counter** | Option<**f64**> | This is the number of free phone numbers the subscription has | [optional]
**phone_numbers_included** | Option<**f64**> | This is the maximum number of free phone numbers the subscription can have | [optional]
**concurrency_limit_purchased** | **f64** | This is the purchased add-on concurrency limit for the subscription. | 
**monthly_charge_schedule_id** | Option<**f64**> | This is the ID of the monthly job that charges for subscription add ons and phone numbers. | [optional]
**monthly_credit_check_schedule_id** | Option<**f64**> | This is the ID of the monthly job that checks whether the credit balance of the subscription is sufficient for the monthly charge. | [optional]
**stripe_customer_id** | Option<**String**> | This is the Stripe customer ID. | [optional]
**stripe_payment_method_id** | Option<**String**> | This is the Stripe payment ID. | [optional]
**slack_support_enabled** | Option<**bool**> | If this flag is true, then the user has purchased slack support. | [optional]
**slack_channel_id** | Option<**String**> | If this subscription has a slack support subscription, the slack channel's ID will be stored here. | [optional]
**hipaa_enabled** | Option<**bool**> | This is the HIPAA enabled flag for the subscription. It determines whether orgs under this subscription have the option to enable HIPAA compliance. | [optional]
**hipaa_common_paper_agreement_id** | Option<**String**> | This is the ID for the Common Paper agreement outlining the HIPAA contract. | [optional]
**stripe_payment_method_fingerprint** | Option<**String**> | This is the Stripe fingerprint of the payment method (card). It allows us to detect users who try to abuse our system through multiple sign-ups. | [optional]
**stripe_customer_email** | Option<**String**> | This is the customer's email on Stripe. | [optional]
**referred_by_email** | Option<**String**> | This is the email of the referrer for the subscription. | [optional]
**auto_reload_plan** | Option<[**models::AutoReloadPlan**](AutoReloadPlan.md)> |  | [optional]
**minutes_included** | Option<**f64**> | The number of minutes included in the subscription. | [optional]
**minutes_used** | Option<**f64**> | The number of minutes used in the subscription. | [optional]
**minutes_used_next_reset_at** | Option<**String**> | This is the timestamp at which the number of monthly free minutes is scheduled to reset at. | [optional]
**minutes_overage_cost** | Option<**f64**> | The per minute charge on minutes that exceed the included minutes. Enterprise only. | [optional]
**providers_included** | Option<**Vec<String>**> | The list of providers included in the subscription. Enterprise only. | [optional]
**outbound_calls_daily_limit** | Option<**f64**> | The maximum number of outbound calls this subscription may make in a day. Resets every night. | [optional]
**outbound_calls_counter** | Option<**f64**> | The current number of outbound calls the subscription has made in the current day. | [optional]
**outbound_calls_counter_next_reset_at** | Option<**String**> | This is the timestamp at which the outbound calls counter is scheduled to reset at. | [optional]
**coupon_ids** | Option<**Vec<String>**> | This is the IDs of the coupons applicable to this subscription. | [optional]
**coupon_usage_left** | Option<**String**> | This is the number of credits left obtained from a coupon. | [optional]
**invoice_plan** | Option<[**models::InvoicePlan**](InvoicePlan.md)> |  | [optional]
**pci_enabled** | Option<**bool**> | This is the PCI enabled flag for the subscription. It determines whether orgs under this subscription have the option to enable PCI compliance. | [optional]
**pci_common_paper_agreement_id** | Option<**String**> | This is the ID for the Common Paper agreement outlining the PCI contract. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


