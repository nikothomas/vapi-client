# UpdateOrgDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hipaa_enabled** | Option<**bool**> | When this is enabled, no logs, recordings, or transcriptions will be stored. At the end of the call, you will still receive an end-of-call-report message to store on your server. Defaults to false. When HIPAA is enabled, only OpenAI/Custom LLM or Azure Providers will be available for LLM and Voice respectively. This is due to the compliance requirements of HIPAA. Other providers may not meet these requirements. | [optional]
**subscription_id** | Option<**String**> | This is the ID of the subscription the org belongs to. | [optional]
**name** | Option<**String**> | This is the name of the org. This is just for your own reference. | [optional]
**channel** | Option<[**models::UpdateOrgDtoChannel**](UpdateOrgDtoChannel.md)> |  | [optional]
**billing_limit** | Option<**f64**> | This is the monthly billing limit for the org. To go beyond $1000/mo, please contact us at support@vapi.ai. | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]
**concurrency_limit** | Option<**f64**> | This is the concurrency limit for the org. This is the maximum number of calls that can be active at any given time. To go beyond 10, please contact us at support@vapi.ai. | [optional]
**compliance_plan** | Option<[**models::CompliancePlan**](CompliancePlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


