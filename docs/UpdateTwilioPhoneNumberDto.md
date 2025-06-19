# UpdateTwilioPhoneNumberDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::ImportTwilioPhoneNumberDtoFallbackDestination**](ImportTwilioPhoneNumberDTO_fallbackDestination.md)> |  | [optional]
**hooks** | Option<[**Vec<models::ImportTwilioPhoneNumberDtoHooksInner>**](ImportTwilioPhoneNumberDTO_hooks_inner.md)> | This is the hooks that will be used for incoming calls to this phone number. | [optional]
**sms_enabled** | Option<**bool**> | Controls whether Vapi sets the messaging webhook URL on the Twilio number during import.  If set to `false`, Vapi will not update the Twilio messaging URL, leaving it as is. If `true` or omitted (default), Vapi will configure both the voice and messaging URLs.  @default true | [optional][default to true]
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId`, `squadId` nor `workflowId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**workflow_id** | Option<**String**> | This is the workflow that will be used for incoming calls to this phone number.  If neither `assistantId`, `squadId`, nor `workflowId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId`, `squadId`, nor `workflowId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> | This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server 2. phoneNumber.server 3. org.server | [optional]
**number** | Option<**String**> | These are the digits of the phone number you own on your Twilio. | [optional]
**twilio_account_sid** | Option<**String**> | This is the Twilio Account SID for the phone number. | [optional]
**twilio_auth_token** | Option<**String**> | This is the Twilio Auth Token for the phone number. | [optional]
**twilio_api_key** | Option<**String**> | This is the Twilio API Key for the phone number. | [optional]
**twilio_api_secret** | Option<**String**> | This is the Twilio API Secret for the phone number. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


