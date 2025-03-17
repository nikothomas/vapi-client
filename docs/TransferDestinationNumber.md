# TransferDestinationNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<[**models::TransferDestinationAssistantMessage**](TransferDestinationAssistant_message.md)> |  | [optional]
**r#type** | **String** |  | 
**number_e164_check_enabled** | Option<**bool**> | This is the flag to toggle the E164 check for the `number` field. This is an advanced property which should be used if you know your use case requires it.  Use cases: - `false`: To allow non-E164 numbers like `+001234567890`, `1234`, or `abc`. This is useful for dialing out to non-E164 numbers on your SIP trunks. - `true` (default): To allow only E164 numbers like `+14155551234`. This is standard for PSTN calls.  If `false`, the `number` is still required to only contain alphanumeric characters (regex: `/^\\+?[a-zA-Z0-9]+$/`).  @default true (E164 check is enabled) | [optional][default to true]
**number** | **String** | This is the phone number to transfer the call to. | 
**extension** | Option<**String**> | This is the extension to dial after transferring the call to the `number`. | [optional]
**caller_id** | Option<**String**> | This is the caller ID to use when transferring the call to the `number`.  Usage: - If not provided, the caller ID will be the number the call is coming from. Example, +14151111111 calls in to and the assistant transfers out to +16470000000. +16470000000 will see +14151111111 as the caller. - To change this behavior, provide a `callerId`. - Set to '{{customer.number}}' to always use the customer's number as the caller ID. - Set to '{{phoneNumber.number}}' to always use the phone number of the assistant as the caller ID. - Set to any E164 number to always use that number as the caller ID. This needs to be a number that is owned or verified by your Transport provider like Twilio.  For Twilio, you can read up more here: https://www.twilio.com/docs/voice/twiml/dial#callerid | [optional]
**transfer_plan** | Option<[**models::TransferPlan**](TransferPlan.md)> | This configures how transfer is executed and the experience of the destination party receiving the call. Defaults to `blind-transfer`.  @default `transferPlan.mode='blind-transfer'` | [optional]
**description** | Option<**String**> | This is the description of the destination, used by the AI to choose when and how to transfer the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


