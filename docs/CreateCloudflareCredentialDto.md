# CreateCloudflareCredentialDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | Credential provider. Only allowed value is cloudflare | 
**account_id** | Option<**String**> | Cloudflare Account Id. | [optional]
**api_key** | Option<**String**> | Cloudflare API Key / Token. | [optional]
**account_email** | Option<**String**> | Cloudflare Account Email. | [optional]
**bucket_plan** | Option<[**models::CloudflareR2BucketPlan**](CloudflareR2BucketPlan.md)> | This is the bucket plan that can be provided to store call artifacts in R2 | [optional]
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


