# Org

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hipaa_enabled** | Option<**bool**> | When this is enabled, no logs, recordings, or transcriptions will be stored. At the end of the call, you will still receive an end-of-call-report message to store on your server. Defaults to false. When HIPAA is enabled, only OpenAI/Custom LLM or Azure Providers will be available for LLM and Voice respectively. This is due to the compliance requirements of HIPAA. Other providers may not meet these requirements. | [optional]
**subscription** | Option<[**models::Subscription**](Subscription.md)> |  | [optional]
**subscription_id** | Option<**String**> | This is the ID of the subscription the org belongs to. | [optional]
**id** | **String** | This is the unique identifier for the org. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the org was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the org was last updated. | 
**stripe_customer_id** | Option<**String**> | This is the Stripe customer for the org. | [optional]
**stripe_subscription_id** | Option<**String**> | This is the subscription for the org. | [optional]
**stripe_subscription_item_id** | Option<**String**> | This is the subscription's subscription item. | [optional]
**stripe_subscription_current_period_start** | Option<**String**> | This is the subscription's current period start. | [optional]
**stripe_subscription_status** | Option<**String**> | This is the subscription's status. | [optional]
**plan** | Option<[**models::OrgPlan**](OrgPlan.md)> | This is the plan for the org. | [optional]
**name** | Option<**String**> | This is the name of the org. This is just for your own reference. | [optional]
**channel** | Option<**String**> | This is the channel of the org. There is the cluster the API traffic for the org will be directed. | [optional]
**billing_limit** | Option<**f64**> | This is the monthly billing limit for the org. To go beyond $1000/mo, please contact us at support@vapi.ai. | [optional]
**server** | Option<[**models::Server**](Server.md)> | This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server 2. phoneNumber.server 3. org.server | [optional]
**concurrency_limit** | Option<**f64**> | This is the concurrency limit for the org. This is the maximum number of calls that can be active at any given time. To go beyond 10, please contact us at support@vapi.ai. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


