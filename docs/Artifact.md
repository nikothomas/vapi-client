# Artifact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<models::ArtifactMessagesItem>**](ArtifactMessagesItem.md)> | These are the messages that were spoken during the call. | [optional]
**messages_open_ai_formatted** | Option<[**Vec<models::OpenAiMessage>**](OpenAiMessage.md)> | These are the messages that were spoken during the call, formatted for OpenAI. | [optional]
**recording_url** | Option<**String**> | This is the recording url for the call. To enable, set `assistant.artifactPlan.recordingEnabled`. | [optional]
**stereo_recording_url** | Option<**String**> | This is the stereo recording url for the call. To enable, set `assistant.artifactPlan.recordingEnabled`. | [optional]
**video_recording_url** | Option<**String**> | This is video recording url for the call. To enable, set `assistant.artifactPlan.videoRecordingEnabled`. | [optional]
**video_recording_start_delay_seconds** | Option<**f64**> | This is video recording start delay in ms. To enable, set `assistant.artifactPlan.videoRecordingEnabled`. This can be used to align the playback of the recording with artifact.messages timestamps. | [optional]
**transcript** | Option<**String**> | This is the transcript of the call. This is derived from `artifact.messages` but provided for convenience. | [optional]
**pcap_url** | Option<**String**> | This is the packet capture url for the call. This is only available for `phone` type calls where phone number's provider is `vapi` or `byo-phone-number`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


