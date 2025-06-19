# Session

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | This is the unique identifier for the session. | 
**org_id** | **String** | This is the unique identifier for the organization that owns this session. | 
**created_at** | **String** | This is the ISO 8601 timestamp indicating when the session was created. | 
**updated_at** | **String** | This is the ISO 8601 timestamp indicating when the session was last updated. | 
**name** | Option<**String**> | This is a user-defined name for the session. Maximum length is 40 characters. | [optional]
**status** | Option<**String**> | This is the current status of the session. Can be either 'active' or 'completed'. | [optional]
**expiration_seconds** | Option<**f64**> | Session expiration time in seconds. Defaults to 24 hours (86400 seconds) if not set. | [optional]
**assistant_id** | Option<**String**> | This is the ID of the assistant associated with this session. Use this when referencing an existing assistant. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant configuration for this session. Use this when creating a new assistant configuration. If assistantId is provided, this will be ignored. | [optional]
**messages** | Option<[**Vec<models::MessageArrayInner>**](MessageArray_inner.md)> | This is an array of chat messages in the session. | [optional]
**customer** | Option<[**models::CreateCustomerDto**](CreateCustomerDTO.md)> | This is the customer information associated with this session. | [optional]
**phone_number_id** | Option<**String**> | This is the ID of the phone number associated with this session. | [optional]
**phone_number** | Option<[**models::ImportTwilioPhoneNumberDto**](ImportTwilioPhoneNumberDTO.md)> | This is the phone number configuration for this session. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


