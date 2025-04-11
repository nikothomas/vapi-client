# TransferPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | [**models::TransferPlanMode**](TransferPlanMode.md) |  | 
**message** | Option<[**models::TransferPlanMessage**](TransferPlanMessage.md)> |  | [optional]
**sip_verb** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This specifies the SIP verb to use while transferring the call. - 'refer': Uses SIP REFER to transfer the call (default) - 'bye': Ends current call with SIP BYE - 'dial': Uses SIP DIAL to transfer the call | [optional]
**twiml** | Option<**String**> | This is the TwiML instructions to execute on the destination call leg before connecting the customer.  Usage: - Used only when `mode` is `warm-transfer-twiml`. - Supports only `Play`, `Say`, `Gather`, `Hangup` and `Pause` verbs. - Maximum length is 4096 characters.  Example: ``` <Say voice=\"alice\" language=\"en-US\">Hello, transferring a customer to you.</Say> <Pause length=\"2\"/> <Say>They called about billing questions.</Say> ``` | [optional]
**summary_plan** | Option<[**models::SummaryPlan**](SummaryPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


