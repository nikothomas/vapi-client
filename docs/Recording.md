# Recording

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stereo_url** | Option<**String**> | This is the stereo recording url for the call. To enable, set `assistant.artifactPlan.recordingEnabled`. | [optional]
**video_url** | Option<**String**> | This is the video recording url for the call. To enable, set `assistant.artifactPlan.videoRecordingEnabled`. | [optional]
**video_recording_start_delay_seconds** | Option<**f64**> | This is video recording start delay in ms. To enable, set `assistant.artifactPlan.videoRecordingEnabled`. This can be used to align the playback of the recording with artifact.messages timestamps. | [optional]
**mono** | Option<[**models::Mono**](Mono.md)> | This is the mono recording url for the call. To enable, set `assistant.artifactPlan.recordingEnabled`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


