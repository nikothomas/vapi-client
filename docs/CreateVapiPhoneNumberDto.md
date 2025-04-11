# CreateVapiPhoneNumberDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::CreateVapiPhoneNumberDtoFallbackDestination**](CreateVapiPhoneNumberDtoFallbackDestination.md)> |  | [optional]
**number_desired_area_code** | Option<**String**> | This is the area code of the phone number to purchase. | [optional]
**sip_uri** | Option<**String**> | This is the SIP URI of the phone number. You can SIP INVITE this. The assistant attached to this number will answer.  This is case-insensitive. | [optional]
**authentication** | Option<[**models::SipAuthentication**](SipAuthentication.md)> |  | [optional]
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


