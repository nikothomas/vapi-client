# TargetPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phone_number_id** | Option<**String**> | This is the phone number that is being tested. During the actual test, it'll be called and the assistant attached to it will pick up and be tested. To test an assistant directly, send assistantId instead. | [optional]
**phone_number** | Option<[**models::TestSuitePhoneNumber**](TestSuitePhoneNumber.md)> | This can be any phone number (even not on Vapi). During the actual test, it'll be called. To test a Vapi number, send phoneNumberId. To test an assistant directly, send assistantId instead. | [optional]
**assistant_id** | Option<**String**> | This is the assistant being tested. During the actual test, it'll invoked directly. To test the assistant over phone number, send phoneNumberId instead. | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | This is the assistant overrides applied to assistantId before it is tested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


