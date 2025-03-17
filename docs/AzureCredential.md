# AzureCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** |  | 
**service** | **String** | This is the service being used in Azure. | [default to Speech]
**region** | Option<**String**> | This is the region of the Azure resource. | [optional]
**api_key** | Option<**String**> | This is not returned in the API. | [optional]
**id** | **String** | This is the unique identifier for the credential. | 
**org_id** | **String** | This is the unique identifier for the org that this credential belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the credential was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]
**bucket_plan** | Option<[**models::AzureBlobStorageBucketPlan**](AzureBlobStorageBucketPlan.md)> | This is the bucket plan that can be provided to store call artifacts in Azure Blob Storage. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


