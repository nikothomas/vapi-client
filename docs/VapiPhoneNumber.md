# VapiPhoneNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::ImportTwilioPhoneNumberDtoFallbackDestination**](ImportTwilioPhoneNumberDTO_fallbackDestination.md)> |  | [optional]
**provider** | **String** | This is to create free SIP phone numbers on Vapi. | 
**id** | **String** | This is the unique identifier for the phone number. | 
**org_id** | **String** | This is the unique identifier for the org that this phone number belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the phone number was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the phone number was last updated. | 
**status** | Option<**String**> | This is the status of the phone number. | [optional]
**number** | Option<**String**> | These are the digits of the phone number you purchased from Vapi. | [optional]
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId` nor `squadId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> | This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server 2. phoneNumber.server 3. org.server | [optional]
**number_desired_area_code** | Option<**String**> | This is the area code of the phone number to purchase. | [optional]
**sip_uri** | Option<**String**> | This is the SIP URI of the phone number. You can SIP INVITE this. The assistant attached to this number will answer.  This is case-insensitive. | [optional]
**authentication** | Option<[**models::SipAuthentication**](SipAuthentication.md)> | This enables authentication for incoming SIP INVITE requests to the `sipUri`.  If not set, any username/password to the 401 challenge of the SIP INVITE will be accepted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


