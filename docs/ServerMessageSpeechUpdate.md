# ServerMessageSpeechUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageSpeechUpdatePhoneNumber**](ServerMessageSpeechUpdatePhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"speech-update\" is sent whenever assistant or user start or stop speaking. | 
**status** | [**models::ServerMessageSpeechUpdateStatus**](ServerMessageSpeechUpdateStatus.md) |  | 
**role** | [**models::ServerMessageSpeechUpdateRole**](ServerMessageSpeechUpdateRole.md) |  | 
**turn** | Option<**f64**> | This is the turn number of the speech update (0-indexed). | [optional]
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


