# TestSuiteRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | This is the current status of the test suite run. | 
**id** | **String** | This is the unique identifier for the test suite run. | 
**org_id** | **String** | This is the unique identifier for the organization this run belongs to. | 
**test_suite_id** | **String** | This is the unique identifier for the test suite this run belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the test suite run was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the test suite run was last updated. | 
**test_results** | [**Vec<models::TestSuiteRunTestResult>**](TestSuiteRunTestResult.md) | These are the results of the tests in this test suite run. | 
**name** | Option<**String**> | This is the name of the test suite run. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


