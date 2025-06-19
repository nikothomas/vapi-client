# ResponseObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for this Response | 
**object** | **String** | The object type | [default to Response]
**created_at** | **f64** | Unix timestamp (in seconds) of when this Response was created | 
**status** | **String** | Status of the response | 
**error** | Option<**String**> | Error message if the response failed | [optional]
**output** | [**Vec<models::ResponseOutputMessage>**](ResponseOutputMessage.md) | Output messages from the model | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


