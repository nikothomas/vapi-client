# ServerMessagePhoneCallControl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessagePhoneCallControlPhoneNumber**](ServerMessagePhoneCallControlPhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"phone-call-control\" is an advanced type of message.  When it is requested in `assistant.serverMessages`, the hangup and forwarding responsibilities are delegated to your server. Vapi will no longer do the actual transfer and hangup. | 
**request** | [**models::ServerMessagePhoneCallControlRequest**](ServerMessagePhoneCallControlRequest.md) |  | 
**destination** | Option<[**models::ServerMessagePhoneCallControlDestination**](ServerMessagePhoneCallControlDestination.md)> |  | [optional]
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


