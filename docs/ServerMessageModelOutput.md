# ServerMessageModelOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number** | Option<[**models::ServerMessageModelOutputPhoneNumber**](ServerMessageModelOutputPhoneNumber.md)> |  | [optional]
**r#type** | **String** | This is the type of the message. \"model-output\" is sent as the model outputs tokens. | 
**timestamp** | Option<**f64**> | This is the timestamp of when the message was sent in milliseconds since Unix Epoch. | [optional]
**artifact** | Option<[**models::Artifact**](Artifact.md)> |  | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDto.md)> |  | [optional]
**call** | Option<[**models::Call**](Call.md)> |  | [optional]
**output** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | This is the output of the model. It can be a token or tool call. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


