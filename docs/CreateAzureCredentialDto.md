# CreateAzureCredentialDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** |  | 
**service** | **String** | This is the service being used in Azure. | [default to Speech]
**region** | Option<**String**> | This is the region of the Azure resource. | [optional]
**api_key** | Option<**String**> | This is not returned in the API. | [optional]
**fallback_index** | Option<**f64**> | This is the order in which this storage provider is tried during upload retries. Lower numbers are tried first in increasing order. | [optional]
**bucket_plan** | Option<[**models::AzureBlobStorageBucketPlan**](AzureBlobStorageBucketPlan.md)> | This is the bucket plan that can be provided to store call artifacts in Azure Blob Storage. | [optional]
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


