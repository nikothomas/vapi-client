# Call

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**models::CallType**](CallType.md)> |  | [optional]
**costs** | Option<[**Vec<models::CallCostsItem>**](CallCostsItem.md)> | These are the costs of individual components of the call in USD. | [optional]
**messages** | Option<[**Vec<models::CallMessagesItem>**](CallMessagesItem.md)> |  | [optional]
**phone_call_provider** | Option<[**models::CallPhoneCallProvider**](CallPhoneCallProvider.md)> |  | [optional]
**phone_call_transport** | Option<[**models::CallPhoneCallTransport**](CallPhoneCallTransport.md)> |  | [optional]
**status** | Option<[**models::CallStatus**](CallStatus.md)> |  | [optional]
**ended_reason** | Option<[**models::CallEndedReason**](CallEndedReason.md)> |  | [optional]
**destination** | Option<[**models::CallDestination**](CallDestination.md)> |  | [optional]
**id** | **String** | This is the unique identifier for the call. | 
**org_id** | **String** | This is the unique identifier for the org that this call belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the call was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the call was last updated. | 
**started_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call was started. | [optional]
**ended_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call was ended. | [optional]
**cost** | Option<**f64**> | This is the cost of the call in USD. | [optional]
**cost_breakdown** | Option<[**models::CostBreakdown**](CostBreakdown.md)> |  | [optional]
**artifact_plan** | Option<[**models::ArtifactPlan**](ArtifactPlan.md)> |  | [optional]
**analysis** | Option<[**models::Analysis**](Analysis.md)> |  | [optional]
**monitor** | Option<[**models::Monitor**](Monitor.md)> |  | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**transport** | Option<[**models::Transport**](Transport.md)> |  | [optional]
**phone_call_provider_id** | Option<**String**> | The ID of the call as provided by the phone number service. callSid in Twilio. conversationUuid in Vonage. callControlId in Telnyx.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> |  | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for the call. To use a transient squad, use `squad` instead. | [optional]
**squad** | Option<[**models::CreateSquadDto**](CreateSquadDto.md)> |  | [optional]
**phone_number_id** | Option<**String**> | This is the phone number that will be used for the call. To use a transient number, use `phoneNumber` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**phone_number** | Option<[**models::ImportTwilioPhoneNumberDto**](ImportTwilioPhoneNumberDto.md)> |  | [optional]
**customer_id** | Option<**String**> | This is the customer that will be called. To call a transient customer , use `customer` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**name** | Option<**String**> | This is the name of the call. This is just for your own reference. | [optional]
**schedule_plan** | Option<[**models::SchedulePlan**](SchedulePlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


