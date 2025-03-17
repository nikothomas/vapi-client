# VapiCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of cost, always 'vapi' for this class. | 
**sub_type** | **String** | This is the sub type of the cost. | 
**minutes** | **f64** | This is the minutes of Vapi usage. This should match `call.endedAt` - `call.startedAt`. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


