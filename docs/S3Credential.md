# S3Credential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | Credential provider. Only allowed value is s3 | 
**aws_access_key_id** | **String** | AWS access key ID. | 
**aws_secret_access_key** | **String** | AWS access key secret. This is not returned in the API. | 
**region** | **String** | AWS region in which the S3 bucket is located. | 
**s3_bucket_name** | **String** | AWS S3 bucket name. | 
**s3_path_prefix** | **String** | The path prefix for the uploaded recording. Ex. \"recordings/\" | 
**id** | **String** | This is the unique identifier for the credential. | 
**org_id** | **String** | This is the unique identifier for the org that this credential belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the credential was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


