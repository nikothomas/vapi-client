# AzureBlobStorageBucketPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_string** | **String** | This is the blob storage connection string for the Azure resource. | 
**container_name** | **String** | This is the container name for the Azure blob storage. | 
**path** | Option<**String**> | This is the path where call artifacts will be stored.  Usage: - To store call artifacts in a specific folder, set this to the full path. Eg. \"/folder-name1/folder-name2\". - To store call artifacts in the root of the bucket, leave this blank.  @default \"/\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


