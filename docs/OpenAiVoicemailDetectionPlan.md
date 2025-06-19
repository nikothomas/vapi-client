# OpenAiVoicemailDetectionPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**beep_max_await_seconds** | Option<**f64**> | This is the maximum duration from the start of the call that we will wait for a voicemail beep, before speaking our message  - If we detect a voicemail beep before this, we will speak the message at that point.  - Setting too low a value means that the bot will start speaking its voicemail message too early. If it does so before the actual beep, it will get cut off. You should definitely tune this to your use case.  @default 30 @min 0 @max 60 | [optional][default to 30]
**provider** | **String** | This is the provider to use for voicemail detection. | 
**backoff_plan** | Option<[**models::VoicemailDetectionBackoffPlan**](VoicemailDetectionBackoffPlan.md)> | This is the backoff plan for the voicemail detection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


