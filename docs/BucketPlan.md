# BucketPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | This is the name of the bucket. | 
**region** | Option<**String**> | This is the region of the bucket.  Usage: - If `credential.type` is `aws`, then this is required. - If `credential.type` is `gcp`, then this is optional since GCP allows buckets to be accessed without a region but region is required for data residency requirements. Read here: https://cloud.google.com/storage/docs/request-endpoints | [optional]
**path** | Option<**String**> | This is the path where call artifacts will be stored.  Usage: - To store call artifacts in a specific folder, set this to the full path. Eg. \"/folder-name1/folder-name2\". - To store call artifacts in the root of the bucket, leave this blank.  @default \"/\" | [optional]
**hmac_access_key** | Option<**String**> | This is the HMAC access key offered by GCP for interoperability with S3 clients. Here is the guide on how to create: https://cloud.google.com/storage/docs/authentication/managing-hmackeys#console  Usage: - If `credential.type` is `gcp`, then this is required. - If `credential.type` is `aws`, then this is not required since credential.awsAccessKeyId is used instead. | [optional]
**hmac_secret** | Option<**String**> | This is the secret for the HMAC access key. Here is the guide on how to create: https://cloud.google.com/storage/docs/authentication/managing-hmackeys#console  Usage: - If `credential.type` is `gcp`, then this is required. - If `credential.type` is `aws`, then this is not required since credential.awsSecretAccessKey is used instead.  Note: This is not returned in the API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


