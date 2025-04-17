# TransferPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | **String** | This configures how transfer is executed and the experience of the destination party receiving the call.  Usage: - `blind-transfer`: The assistant forwards the call to the destination without any message or summary. - `blind-transfer-add-summary-to-sip-header`: The assistant forwards the call to the destination and adds a SIP header X-Transfer-Summary to the call to include the summary. - `warm-transfer-say-message`: The assistant dials the destination, delivers the `message` to the destination party, connects the customer, and leaves the call. - `warm-transfer-say-summary`: The assistant dials the destination, provides a summary of the call to the destination party, connects the customer, and leaves the call. - `warm-transfer-wait-for-operator-to-speak-first-and-then-say-message`: The assistant dials the destination, waits for the operator to speak, delivers the `message` to the destination party, and then connects the customer. - `warm-transfer-wait-for-operator-to-speak-first-and-then-say-summary`: The assistant dials the destination, waits for the operator to speak, provides a summary of the call to the destination party, and then connects the customer. - `warm-transfer-twiml`: The assistant dials the destination, executes the twiml instructions on the destination call leg, connects the customer, and leaves the call.  @default 'blind-transfer' | 
**message** | Option<[**models::TransferPlanMessage**](TransferPlan_message.md)> |  | [optional]
**sip_verb** | Option<**String**> | This specifies the SIP verb to use while transferring the call. - 'refer': Uses SIP REFER to transfer the call (default) - 'bye': Ends current call with SIP BYE - 'dial': Uses SIP DIAL to transfer the call | [optional]
**twiml** | Option<**String**> | This is the TwiML instructions to execute on the destination call leg before connecting the customer.  Usage: - Used only when `mode` is `warm-transfer-twiml`. - Supports only `Play`, `Say`, `Gather`, `Hangup` and `Pause` verbs. - Maximum length is 4096 characters.  Example: ``` <Say voice=\"alice\" language=\"en-US\">Hello, transferring a customer to you.</Say> <Pause length=\"2\"/> <Say>They called about billing questions.</Say> ``` | [optional]
**summary_plan** | Option<[**models::SummaryPlan**](SummaryPlan.md)> | This is the plan for generating a summary of the call to present to the destination party.  Usage: - Used only when `mode` is `blind-transfer-add-summary-to-sip-header` or `warm-transfer-say-summary` or `warm-transfer-wait-for-operator-to-speak-first-and-then-say-summary`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


