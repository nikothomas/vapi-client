# VoicemailDetectionBackoffPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_at_seconds** | Option<**f64**> | This is the number of seconds to wait before starting the first retry attempt. | [optional][default to 5]
**frequency_seconds** | Option<**f64**> | This is the interval in seconds between retry attempts. | [optional][default to 5]
**max_retries** | Option<**f64**> | This is the maximum number of retry attempts before giving up. | [optional][default to 6]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


