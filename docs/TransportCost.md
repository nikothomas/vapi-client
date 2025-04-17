# TransportCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of cost, always 'transport' for this class. | 
**provider** | Option<**String**> |  | [optional]
**minutes** | **f64** | This is the minutes of `transport` usage. This should match `call.endedAt` - `call.startedAt`. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


