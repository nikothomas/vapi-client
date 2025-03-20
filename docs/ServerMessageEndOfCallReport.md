# ServerMessageEndOfCallReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageAssistantRequestPhoneNumber**](ServerMessageAssistantRequest_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"end-of-call-report\" is sent when the call ends and post-processing is complete. | 
**ended_reason** | **String** | This is the reason the call ended. This can also be found at `call.endedReason` on GET /call/:id. | 
**cost** | Option<**f64**> | This is the cost of the call in USD. This can also be found at `call.cost` on GET /call/:id. | [optional]
**costs** | Option<[**Vec<models::CallCostsInner>**](Call_costs_inner.md)> | These are the costs of individual components of the call in USD. This can also be found at `call.costs` on GET /call/:id. | [optional]
**timestamp** | Option<**f64**> | This is the ISO-8601 formatted timestamp of when the message was sent. | [optional]
**artifact** | [**models::Artifact**](Artifact.md) | These are the artifacts from the call. This can also be found at `call.artifact` on GET /call/:id. | 
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that is currently active. This is provided for convenience.  This matches one of the following: - `call.assistant`, - `call.assistantId`, - `call.squad[n].assistant`, - `call.squad[n].assistantId`, - `call.squadId->[n].assistant`, - `call.squadId->[n].assistantId`. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer associated with the call.  This matches one of the following: - `call.customer`, - `call.customerId`. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call object.  This matches what was returned in POST /call.  Note: This might get stale during the call. To get the latest call object, especially after the call is ended, use GET /call/:id. | [optional]
**analysis** | [**models::Analysis**](Analysis.md) | This is the analysis of the call. This can also be found at `call.analysis` on GET /call/:id. | 
**started_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call started. This can also be found at `call.startedAt` on GET /call/:id. | [optional]
**ended_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call ended. This can also be found at `call.endedAt` on GET /call/:id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


