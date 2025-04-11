# ServerMessageHang

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageHangPhoneNumber**](ServerMessageHangPhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"hang\" is sent when the assistant is hanging due to a delay. The delay can be caused by many factors, such as: - the model is too slow to respond - the voice is too slow to respond - the tool call is still waiting for a response from your server - etc. | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


