# ServerMessageTranscript

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageAssistantRequestPhoneNumber**](ServerMessageAssistantRequest_phoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"transcript\" is sent as transcriber outputs partial or final transcript. | 
**timestamp** | Option<**f64**> | This is the ISO-8601 formatted timestamp of when the message was sent. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> | This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that is currently active. This is provided for convenience.  This matches one of the following: - `call.assistant`, - `call.assistantId`, - `call.squad[n].assistant`, - `call.squad[n].assistantId`, - `call.squadId->[n].assistant`, - `call.squadId->[n].assistantId`. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer associated with the call.  This matches one of the following: - `call.customer`, - `call.customerId`. | [optional]
**call** | Option<[**models::Call**](Call.md)> | This is the call object.  This matches what was returned in POST /call.  Note: This might get stale during the call. To get the latest call object, especially after the call is ended, use GET /call/:id. | [optional]
**role** | **String** | This is the role for which the transcript is for. | 
**transcript_type** | **String** | This is the type of the transcript. | 
**transcript** | **String** | This is the transcript content. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


