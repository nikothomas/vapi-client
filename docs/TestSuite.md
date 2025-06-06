# TestSuite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | This is the unique identifier for the test suite. | 
**org_id** | **String** | This is the unique identifier for the org that this test suite belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the test suite was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the test suite was last updated. | 
**name** | Option<**String**> | This is the name of the test suite. | [optional]
**phone_number_id** | Option<**String**> | This is the phone number ID associated with this test suite. | [optional]
**tester_plan** | Option<[**models::TesterPlan**](TesterPlan.md)> | Override the default tester plan by providing custom assistant configuration for the test agent.  We recommend only using this if you are confident, as we have already set sensible defaults on the tester plan. | [optional]
**target_plan** | Option<[**models::TargetPlan**](TargetPlan.md)> | These are the configuration for the assistant / phone number that is being tested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


