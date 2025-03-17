# TransportConfigurationTwilio

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** |  | 
**timeout** | Option<**f64**> | The integer number of seconds that we should allow the phone to ring before assuming there is no answer. The default is `60` seconds and the maximum is `600` seconds. For some call flows, we will add a 5-second buffer to the timeout value you provide. For this reason, a timeout value of 10 seconds could result in an actual timeout closer to 15 seconds. You can set this to a short time, such as `15` seconds, to hang up before reaching an answering machine or voicemail.  @default 60 | [optional]
**record** | Option<**bool**> | Whether to record the call. Can be `true` to record the phone call, or `false` to not. The default is `false`.  @default false | [optional]
**recording_channels** | Option<**String**> | The number of channels in the final recording. Can be: `mono` or `dual`. The default is `mono`. `mono` records both legs of the call in a single channel of the recording file. `dual` records each leg to a separate channel of the recording file. The first channel of a dual-channel recording contains the parent call and the second channel contains the child call.  @default 'mono' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


