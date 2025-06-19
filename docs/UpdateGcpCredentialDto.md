# UpdateGcpCredentialDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_index** | Option<**f64**> | This is the order in which this storage provider is tried during upload retries. Lower numbers are tried first in increasing order. | [optional]
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]
**gcp_key** | Option<[**models::GcpKey**](GcpKey.md)> | This is the GCP key. This is the JSON that can be generated in the Google Cloud Console at https://console.cloud.google.com/iam-admin/serviceaccounts/details/<service-account-id>/keys.  The schema is identical to the JSON that GCP outputs. | [optional]
**region** | Option<**String**> | This is the region of the GCP resource. | [optional]
**bucket_plan** | Option<[**models::BucketPlan**](BucketPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


