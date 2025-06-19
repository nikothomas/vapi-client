# UpdateSessionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | This is the new name for the session. Maximum length is 40 characters. | [optional]
**status** | Option<**String**> | This is the new status for the session. | [optional]
**expiration_seconds** | Option<**f64**> | Session expiration time in seconds. Defaults to 24 hours (86400 seconds) if not set. | [optional]
**messages** | Option<[**Vec<models::MessageArrayInner>**](MessageArray_inner.md)> | This is the updated array of chat messages. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


