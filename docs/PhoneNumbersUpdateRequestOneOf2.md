# PhoneNumbersUpdateRequestOneOf2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::UpdateVonagePhoneNumberDtoFallbackDestination**](UpdateVonagePhoneNumberDtoFallbackDestination.md)> |  | [optional]
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]
**number** | Option<**String**> | These are the digits of the phone number you own on your Vonage. | [optional]
**credential_id** | Option<**String**> | This is the credential you added in dashboard.vapi.ai/keys. This is used to configure the number to send inbound calls to Vapi, make outbound calls and do live call updates like transfers and hangups. | [optional]
**provider** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


