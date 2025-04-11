# PhoneNumbersCreateRequestOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::CreateByoPhoneNumberDtoFallbackDestination**](CreateByoPhoneNumberDtoFallbackDestination.md)> |  | [optional]
**number_e164_check_enabled** | Option<**bool**> | This is the flag to toggle the E164 check for the `number` field. This is an advanced property which should be used if you know your use case requires it.  Use cases: - `false`: To allow non-E164 numbers like `+001234567890`, `1234`, or `abc`. This is useful for dialing out to non-E164 numbers on your SIP trunks. - `true` (default): To allow only E164 numbers like `+14155551234`. This is standard for PSTN calls.  If `false`, the `number` is still required to only contain alphanumeric characters (regex: `/^\\+?[a-zA-Z0-9]+$/`).  @default true (E164 check is enabled) | [optional]
**number** | Option<**String**> | This is the number of the customer. | [optional]
**credential_id** | **String** | This is the credential of your own SIP trunk or Carrier (type `byo-sip-trunk`) which can be used to make calls to this phone number.  You can add the SIP trunk or Carrier credential in the Provider Credentials page on the Dashboard to get the credentialId. | 
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]
**provider** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


