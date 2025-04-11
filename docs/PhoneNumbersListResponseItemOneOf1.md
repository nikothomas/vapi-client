# PhoneNumbersListResponseItemOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::TwilioPhoneNumberFallbackDestination**](TwilioPhoneNumberFallbackDestination.md)> |  | [optional]
**id** | **String** | This is the unique identifier for the phone number. | 
**org_id** | **String** | This is the unique identifier for the org that this phone number belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the phone number was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the phone number was last updated. | 
**status** | Option<[**models::TwilioPhoneNumberStatus**](TwilioPhoneNumberStatus.md)> |  | [optional]
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]
**number** | **String** | These are the digits of the phone number you own on your Twilio. | 
**twilio_account_sid** | **String** | This is the Twilio Account SID for the phone number. | 
**twilio_auth_token** | **String** | This is the Twilio Auth Token for the phone number. | 
**provider** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


