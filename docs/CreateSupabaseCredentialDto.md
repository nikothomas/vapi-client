# CreateSupabaseCredentialDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is for supabase storage. | 
**fallback_index** | Option<**f64**> | This is the order in which this storage provider is tried during upload retries. Lower numbers are tried first in increasing order. | [optional]
**bucket_plan** | Option<[**models::SupabaseBucketPlan**](SupabaseBucketPlan.md)> |  | [optional]
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


