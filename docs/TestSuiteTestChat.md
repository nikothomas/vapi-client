# TestSuiteTestChat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scorers** | [**Vec<models::TestSuiteTestVoiceScorersInner>**](TestSuiteTestVoice_scorers_inner.md) | These are the scorers used to evaluate the test. | 
**r#type** | **String** | This is the type of the test, which must be chat. | 
**id** | **String** | This is the unique identifier for the test. | 
**test_suite_id** | **String** | This is the unique identifier for the test suite this test belongs to. | 
**org_id** | **String** | This is the unique identifier for the organization this test belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the test was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the test was last updated. | 
**name** | Option<**String**> | This is the name of the test. | [optional]
**script** | **String** | This is the script to be used for the chat test. | 
**num_attempts** | Option<**f64**> | This is the number of attempts allowed for the test. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


