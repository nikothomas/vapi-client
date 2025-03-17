# CloudflareR2BucketPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> | Cloudflare R2 Access key ID. | [optional]
**secret_access_key** | Option<**String**> | Cloudflare R2 access key secret. This is not returned in the API. | [optional]
**url** | Option<**String**> | Cloudflare R2 base url. | [optional]
**name** | **String** | This is the name of the bucket. | 
**path** | Option<**String**> | This is the path where call artifacts will be stored.  Usage: - To store call artifacts in a specific folder, set this to the full path. Eg. \"/folder-name1/folder-name2\". - To store call artifacts in the root of the bucket, leave this blank.  @default \"/\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


