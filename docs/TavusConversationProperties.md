# TavusConversationProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_call_duration** | Option<**f64**> | The maximum duration of the call in seconds. The default `maxCallDuration` is 3600 seconds (1 hour). Once the time limit specified by this parameter has been reached, the conversation will automatically shut down. | [optional]
**participant_left_timeout** | Option<**f64**> | The duration in seconds after which the call will be automatically shut down once the last participant leaves. | [optional]
**participant_absent_timeout** | Option<**f64**> | Starting from conversation creation, the duration in seconds after which the call will be automatically shut down if no participant joins the call. Default is 300 seconds (5 minutes). | [optional]
**enable_recording** | Option<**bool**> | If true, the user will be able to record the conversation. | [optional]
**enable_transcription** | Option<**bool**> | If true, the user will be able to transcribe the conversation. You can find more instructions on displaying transcriptions if you are using your custom DailyJS components here. You need to have an event listener on Daily that listens for `app-messages`. | [optional]
**apply_greenscreen** | Option<**bool**> | If true, the background will be replaced with a greenscreen (RGB values: `[0, 255, 155]`). You can use WebGL on the frontend to make the greenscreen transparent or change its color. | [optional]
**language** | Option<**String**> | The language of the conversation. Please provide the **full language name**, not the two-letter code. If you are using your own TTS voice, please ensure it supports the language you provide. If you are using a stock replica or default persona, please note that only ElevenLabs and Cartesia supported languages are available. You can find a full list of supported languages for Cartesia here, for ElevenLabs here, and for PlayHT here. | [optional]
**recording_s3_bucket_name** | Option<**String**> | The name of the S3 bucket where the recording will be stored. | [optional]
**recording_s3_bucket_region** | Option<**String**> | The region of the S3 bucket where the recording will be stored. | [optional]
**aws_assume_role_arn** | Option<**String**> | The ARN of the role that will be assumed to access the S3 bucket. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


