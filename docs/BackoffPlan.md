# BackoffPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of backoff plan to use. Defaults to fixed.  @default fixed | 
**max_retries** | **f64** | This is the maximum number of retries to attempt if the request fails. Defaults to 0 (no retries).  @default 0 | 
**base_delay_seconds** | **f64** | This is the base delay in seconds. For linear backoff, this is the delay between each retry. For exponential backoff, this is the initial delay. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


