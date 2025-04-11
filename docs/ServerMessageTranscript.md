# ServerMessageTranscript

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageTranscriptPhoneNumber**](ServerMessageTranscriptPhoneNumber.md)> |  | [optional]
**r#type** | [**models::ServerMessageTranscriptType**](ServerMessageTranscriptType.md) |  | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]
**role** | [**models::ServerMessageTranscriptRole**](ServerMessageTranscriptRole.md) |  | 
**transcript_type** | [**models::ServerMessageTranscriptTranscriptType**](ServerMessageTranscriptTranscriptType.md) |  | 
**transcript** | **String** | This is the transcript content. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


