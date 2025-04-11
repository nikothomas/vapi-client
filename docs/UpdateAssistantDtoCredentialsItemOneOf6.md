# UpdateAssistantDtoCredentialsItemOneOf6

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gateways** | [**Vec<models::SipTrunkGateway>**](SipTrunkGateway.md) | This is the list of SIP trunk's gateways. | 
**outbound_authentication_plan** | Option<[**models::SipTrunkOutboundAuthenticationPlan**](SipTrunkOutboundAuthenticationPlan.md)> |  | [optional]
**outbound_leading_plus_enabled** | Option<**bool**> | This ensures the outbound origination attempts have a leading plus. Defaults to false to match conventional telecom behavior.  Usage: - Vonage/Twilio requires leading plus for all outbound calls. Set this to true.  @default false | [optional]
**tech_prefix** | Option<**String**> | This can be used to configure the tech prefix on outbound calls. This is an advanced property. | [optional]
**sip_diversion_header** | Option<**String**> | This can be used to enable the SIP diversion header for authenticating the calling number if the SIP trunk supports it. This is an advanced property. | [optional]
**sbc_configuration** | Option<[**serde_json::Value**](.md)> |  | [optional]
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]
**provider** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


