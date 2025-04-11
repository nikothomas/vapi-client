# ArtifactPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recording_enabled** | Option<**bool**> | This determines whether assistant's calls are recorded. Defaults to true.  Usage: - If you don't want to record the calls, set this to false. - If you want to record the calls when `assistant.hipaaEnabled` (deprecated) or `assistant.compliancePlan.hipaaEnabled` explicity set this to true and make sure to provide S3 or GCP credentials on the Provider Credentials page in the Dashboard.  You can find the recording at `call.artifact.recordingUrl` and `call.artifact.stereoRecordingUrl` after the call is ended.  @default true | [optional]
**recording_format** | Option<[**models::ArtifactPlanRecordingFormat**](ArtifactPlanRecordingFormat.md)> |  | [optional]
**video_recording_enabled** | Option<**bool**> | This determines whether the video is recorded during the call. Defaults to false. Only relevant for `webCall` type.  You can find the video recording at `call.artifact.videoRecordingUrl` after the call is ended.  @default false | [optional]
**pcap_enabled** | Option<**bool**> | This determines whether the SIP packet capture is enabled. Defaults to true. Only relevant for `phone` type calls where phone number's provider is `vapi` or `byo-phone-number`.  You can find the packet capture at `call.artifact.pcapUrl` after the call is ended.  @default true | [optional]
**pcap_s3_path_prefix** | Option<**String**> | This is the path where the SIP packet capture will be uploaded. This is only used if you have provided S3 or GCP credentials on the Provider Credentials page in the Dashboard.  If credential.s3PathPrefix or credential.bucketPlan.path is set, this will append to it.  Usage: - If you want to upload the packet capture to a specific path, set this to the path. Example: `/my-assistant-captures`. - If you want to upload the packet capture to the root of the bucket, set this to `/`.  @default '/' | [optional]
**transcript_plan** | Option<[**models::TranscriptPlan**](TranscriptPlan.md)> |  | [optional]
**recording_path** | Option<**String**> | This is the path where the recording will be uploaded. This is only used if you have provided S3 or GCP credentials on the Provider Credentials page in the Dashboard.  If credential.s3PathPrefix or credential.bucketPlan.path is set, this will append to it.  Usage: - If you want to upload the recording to a specific path, set this to the path. Example: `/my-assistant-recordings`. - If you want to upload the recording to the root of the bucket, set this to `/`.  @default '/' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


