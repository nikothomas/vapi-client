# ByoPhoneNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::ImportTwilioPhoneNumberDtoFallbackDestination**](ImportTwilioPhoneNumberDTO_fallbackDestination.md)> |  | [optional]
**provider** | **String** | This is to bring your own phone numbers from your own SIP trunks or Carriers. | 
**number_e164_check_enabled** | Option<**bool**> | This is the flag to toggle the E164 check for the `number` field. This is an advanced property which should be used if you know your use case requires it.  Use cases: - `false`: To allow non-E164 numbers like `+001234567890`, `1234`, or `abc`. This is useful for dialing out to non-E164 numbers on your SIP trunks. - `true` (default): To allow only E164 numbers like `+14155551234`. This is standard for PSTN calls.  If `false`, the `number` is still required to only contain alphanumeric characters (regex: `/^\\+?[a-zA-Z0-9]+$/`).  @default true (E164 check is enabled) | [optional][default to true]
**id** | **String** | This is the unique identifier for the phone number. | 
**org_id** | **String** | This is the unique identifier for the org that this phone number belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the phone number was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the phone number was last updated. | 
**status** | Option<**String**> | This is the status of the phone number. | [optional]
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> | This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server 2. phoneNumber.server 3. org.server | [optional]
**number** | Option<**String**> | This is the number of the customer. | [optional]
**credential_id** | **String** | This is the credential of your own SIP trunk or Carrier (type `byo-sip-trunk`) which can be used to make calls to this phone number.  You can add the SIP trunk or Carrier credential in the Provider Credentials page on the Dashboard to get the credentialId. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


