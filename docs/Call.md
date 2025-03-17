# Call

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | This is the type of call. | [optional]
**costs** | Option<[**Vec<models::CallCostsInner>**](Call_costs_inner.md)> | These are the costs of individual components of the call in USD. | [optional]
**messages** | Option<[**Vec<models::ArtifactMessagesInner>**](Artifact_messages_inner.md)> |  | [optional]
**phone_call_provider** | Option<**String**> | This is the provider of the call.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**phone_call_transport** | Option<**String**> | This is the transport of the phone call.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**status** | Option<**String**> | This is the status of the call. | [optional]
**ended_reason** | Option<**String**> | This is the explanation for how the call ended. | [optional]
**destination** | Option<[**models::CallDestination**](Call_destination.md)> |  | [optional]
**id** | **String** | This is the unique identifier for the call. | 
**org_id** | **String** | This is the unique identifier for the org that this call belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the call was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the call was last updated. | 
**started_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call was started. | [optional]
**ended_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call was ended. | [optional]
**cost** | Option<**f64**> | This is the cost of the call in USD. | [optional]
**cost_breakdown** | Option<[**models::CostBreakdown**](CostBreakdown.md)> | This is the cost of the call in USD. | [optional]
**artifact_plan** | Option<[**models::ArtifactPlan**](ArtifactPlan.md)> | This is a copy of assistant artifact plan. This isn't actually stored on the call but rather just returned in POST /call/web to enable artifact creation client side. | [optional]
**analysis** | Option<[**models::Analysis**](Analysis.md)> | This is the analysis of the call. Configure in `assistant.analysisPlan`. | [optional]
**monitor** | Option<[**models::Monitor**](Monitor.md)> | This is to real-time monitor the call. Configure in `assistant.monitorPlan`. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | These are the artifacts created from the call. Configure in `assistant.artifactPlan`. | [optional]
**transport** | Option<[**models::Transport**](Transport.md)> | This is the transport used for the call. | [optional]
**phone_call_provider_id** | Option<**String**> | The ID of the call as provided by the phone number service. callSid in Twilio. conversationUuid in Vonage. callControlId in Telnyx.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead. | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | These are the overrides for the `assistant` or `assistantId`'s settings and template variables. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for the call. To use a transient squad, use `squad` instead. | [optional]
**squad** | Option<[**models::CreateSquadDto**](CreateSquadDTO.md)> | This is a squad that will be used for the call. To use an existing squad, use `squadId` instead. | [optional]
**phone_number_id** | Option<**String**> | This is the phone number that will be used for the call. To use a transient number, use `phoneNumber` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**phone_number** | Option<[**models::ImportTwilioPhoneNumberDto**](ImportTwilioPhoneNumberDTO.md)> | This is the phone number that will be used for the call. To use an existing number, use `phoneNumberId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**customer_id** | Option<**String**> | This is the customer that will be called. To call a transient customer , use `customer` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer that will be called. To call an existing customer, use `customerId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type. | [optional]
**name** | Option<**String**> | This is the name of the call. This is just for your own reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


