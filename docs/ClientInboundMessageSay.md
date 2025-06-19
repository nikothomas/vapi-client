# ClientInboundMessageSay

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | This is the type of the message. Send \"say\" message to make the assistant say something. | [optional]
**interrupt_assistant_enabled** | Option<**bool**> | This is the flag for whether the message should replace existing assistant speech.  @default false | [optional][default to false]
**content** | Option<**String**> | This is the content to say. | [optional]
**end_call_after_spoken** | Option<**bool**> | This is the flag to end call after content is spoken. | [optional]
**interruptions_enabled** | Option<**bool**> | This is the flag for whether the message is interruptible by the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


