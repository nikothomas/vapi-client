# CreateCustomerDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**number_e164_check_enabled** | Option<**bool**> | This is the flag to toggle the E164 check for the `number` field. This is an advanced property which should be used if you know your use case requires it.  Use cases: - `false`: To allow non-E164 numbers like `+001234567890`, `1234`, or `abc`. This is useful for dialing out to non-E164 numbers on your SIP trunks. - `true` (default): To allow only E164 numbers like `+14155551234`. This is standard for PSTN calls.  If `false`, the `number` is still required to only contain alphanumeric characters (regex: `/^\\+?[a-zA-Z0-9]+$/`).  @default true (E164 check is enabled) | [optional][default to true]
**extension** | Option<**String**> | This is the extension that will be dialed after the call is answered. | [optional]
**number** | Option<**String**> | This is the number of the customer. | [optional]
**sip_uri** | Option<**String**> | This is the SIP URI of the customer. | [optional]
**name** | Option<**String**> | This is the name of the customer. This is just for your own reference.  For SIP inbound calls, this is extracted from the `From` SIP header with format `\"Display Name\" <sip:username@domain>`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


