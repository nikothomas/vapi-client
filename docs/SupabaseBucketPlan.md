# SupabaseBucketPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | **String** | This is the S3 Region. It should look like us-east-1 It should be one of the supabase regions defined in the SUPABASE_REGION enum Check https://supabase.com/docs/guides/platform/regions for up to date regions | 
**url** | **String** | This is the S3 compatible URL for Supabase S3 This should look like https://<project-ID>.supabase.co/storage/v1/s3 | 
**access_key_id** | **String** | This is the Supabase S3 Access Key ID. The user creates this in the Supabase project Storage settings | 
**secret_access_key** | **String** | This is the Supabase S3 Secret Access Key. The user creates this in the Supabase project Storage settings along with the access key id | 
**name** | **String** | This is the Supabase S3 Bucket Name. The user must create this in Supabase under Storage > Buckets A bucket that does not exist will not be checked now, but file uploads will fail | 
**path** | Option<**String**> | This is the Supabase S3 Bucket Folder Path. The user can create this in Supabase under Storage > Buckets A path that does not exist will not be checked now, but file uploads will fail A Path is like a folder in the bucket Eg. If the bucket is called \"my-bucket\" and the path is \"my-folder\", the full path is \"my-bucket/my-folder\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


