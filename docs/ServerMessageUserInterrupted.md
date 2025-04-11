# ServerMessageUserInterrupted

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageUserInterruptedPhoneNumber**](ServerMessageUserInterruptedPhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"user-interrupted\" is sent when the user interrupts the assistant. | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


