# ClientMessageSpeechUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of the message. \"speech-update\" is sent whenever assistant or user start or stop speaking. | 
**status** | [**models::ClientMessageSpeechUpdateStatus**](ClientMessageSpeechUpdateStatus.md) |  | 
**role** | [**models::ClientMessageSpeechUpdateRole**](ClientMessageSpeechUpdateRole.md) |  | 
**turn** | Option<**f64**> | This is the turn number of the speech update (0-indexed). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


