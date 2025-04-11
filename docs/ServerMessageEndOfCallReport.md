# ServerMessageEndOfCallReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageEndOfCallReportPhoneNumber**](ServerMessageEndOfCallReportPhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"end-of-call-report\" is sent when the call ends and post-processing is complete. | 
**ended_reason** | [**models::ServerMessageEndOfCallReportEndedReason**](ServerMessageEndOfCallReportEndedReason.md) |  | 
**cost** | Option<**f64**> | This is the cost of the call in USD. This can also be found at `call.cost` on GET /call/:id. | [optional]
**costs** | Option<[**Vec<models::ServerMessageEndOfCallReportCostsItem>**](ServerMessageEndOfCallReportCostsItem.md)> | These are the costs of individual components of the call in USD. This can also be found at `call.costs` on GET /call/:id. | [optional]
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | [**models::Artifact**](Artifact.md) |  | 
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]
**analysis** | [**models::Analysis**](Analysis.md) |  | 
**started_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call started. This can also be found at `call.startedAt` on GET /call/:id. | [optional]
**ended_at** | Option<**String**> | This is the ISO 8601 date-time string of when the call ended. This can also be found at `call.endedAt` on GET /call/:id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


