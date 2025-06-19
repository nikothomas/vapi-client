# CreateVonagePhoneNumberDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fallback_destination** | Option<[**models::ImportTwilioPhoneNumberDtoFallbackDestination**](ImportTwilioPhoneNumberDTO_fallbackDestination.md)> |  | [optional]
**hooks** | Option<[**Vec<models::ImportTwilioPhoneNumberDtoHooksInner>**](ImportTwilioPhoneNumberDTO_hooks_inner.md)> | This is the hooks that will be used for incoming calls to this phone number. | [optional]
**provider** | **String** | This is to use numbers bought on Vonage. | 
**number** | **String** | These are the digits of the phone number you own on your Vonage. | 
**credential_id** | **String** | This is the credential you added in dashboard.vapi.ai/keys. This is used to configure the number to send inbound calls to Vapi, make outbound calls and do live call updates like transfers and hangups. | 
**name** | Option<**String**> | This is the name of the phone number. This is just for your own reference. | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for incoming calls to this phone number.  If neither `assistantId`, `squadId` nor `workflowId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**workflow_id** | Option<**String**> | This is the workflow that will be used for incoming calls to this phone number.  If neither `assistantId`, `squadId`, nor `workflowId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for incoming calls to this phone number.  If neither `assistantId`, `squadId`, nor `workflowId` is set, `assistant-request` will be sent to your Server URL. Check `ServerMessage` and `ServerMessageResponse` for the shape of the message and response that is expected. | [optional]
**server** | Option<[**models::Server**](Server.md)> | This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server 2. phoneNumber.server 3. org.server | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


