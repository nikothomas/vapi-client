# MessagePlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idle_messages** | Option<**Vec<String>**> | This are the messages that the assistant will speak when the user hasn't responded for `idleTimeoutSeconds`. Each time the timeout is triggered, a random message will be chosen from this array.  Usage: - If user gets distracted and doesn't respond for a while, this can be used to grab their attention. - If the transcriber doesn't pick up what the user said, this can be used to ask the user to repeat themselves. (From the perspective of the assistant, the conversation is idle since it didn't \"hear\" any user messages.)  @default null (no idle message is spoken) | [optional]
**idle_message_max_spoken_count** | Option<**f64**> | This determines the maximum number of times `idleMessages` can be spoken during the call.  @default 3 | [optional]
**idle_message_reset_count_on_user_speech_enabled** | Option<**bool**> | This determines whether the idle message count is reset whenever the user speaks.  @default false | [optional]
**idle_timeout_seconds** | Option<**f64**> | This is the timeout in seconds before a message from `idleMessages` is spoken. The clock starts when the assistant finishes speaking and remains active until the user speaks.  @default 10 | [optional]
**silence_timeout_message** | Option<**String**> | This is the message that the assistant will say if the call ends due to silence.  If unspecified, it will hang up without saying anything. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


