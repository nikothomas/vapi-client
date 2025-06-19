# CloudflareCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | Credential provider. Only allowed value is cloudflare | 
**account_id** | Option<**String**> | Cloudflare Account Id. | [optional]
**api_key** | Option<**String**> | Cloudflare API Key / Token. | [optional]
**account_email** | Option<**String**> | Cloudflare Account Email. | [optional]
**fallback_index** | Option<**f64**> | This is the order in which this storage provider is tried during upload retries. Lower numbers are tried first in increasing order. | [optional]
**id** | **String** | This is the unique identifier for the credential. | 
**org_id** | **String** | This is the unique identifier for the org that this credential belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the credential was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]
**bucket_plan** | Option<[**models::CloudflareR2BucketPlan**](CloudflareR2BucketPlan.md)> | This is the bucket plan that can be provided to store call artifacts in R2 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


