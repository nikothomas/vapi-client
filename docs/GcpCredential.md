# GcpCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** |  | 
**fallback_index** | Option<**f64**> | This is the order in which this storage provider is tried during upload retries. Lower numbers are tried first in increasing order. | [optional]
**id** | **String** | This is the unique identifier for the credential. | 
**org_id** | **String** | This is the unique identifier for the org that this credential belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the credential was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]
**gcp_key** | [**models::GcpKey**](GcpKey.md) | This is the GCP key. This is the JSON that can be generated in the Google Cloud Console at https://console.cloud.google.com/iam-admin/serviceaccounts/details/<service-account-id>/keys.  The schema is identical to the JSON that GCP outputs. | 
**region** | Option<**String**> | This is the region of the GCP resource. | [optional]
**bucket_plan** | Option<[**models::BucketPlan**](BucketPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


