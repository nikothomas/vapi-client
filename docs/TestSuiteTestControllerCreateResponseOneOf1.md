# TestSuiteTestControllerCreateResponseOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scorers** | [**Vec<models::TestSuiteTestScorerAi>**](TestSuiteTestScorerAi.md) | These are the scorers used to evaluate the test. | 
**id** | **String** | This is the unique identifier for the test. | 
**test_suite_id** | **String** | This is the unique identifier for the test suite this test belongs to. | 
**org_id** | **String** | This is the unique identifier for the organization this test belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the test was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the test was last updated. | 
**name** | Option<**String**> | This is the name of the test. | [optional]
**script** | **String** | This is the script to be used for the chat test. | 
**num_attempts** | Option<**f64**> | This is the number of attempts allowed for the test. | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


