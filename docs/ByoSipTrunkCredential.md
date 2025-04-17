# ByoSipTrunkCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | Option<**String**> | This can be used to bring your own SIP trunks or to connect to a Carrier. | [optional]
**id** | **String** | This is the unique identifier for the credential. | 
**org_id** | **String** | This is the unique identifier for the org that this credential belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the credential was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]
**gateways** | [**Vec<models::SipTrunkGateway>**](SipTrunkGateway.md) | This is the list of SIP trunk's gateways. | 
**outbound_authentication_plan** | Option<[**models::SipTrunkOutboundAuthenticationPlan**](SipTrunkOutboundAuthenticationPlan.md)> | This can be used to configure the outbound authentication if required by the SIP trunk. | [optional]
**outbound_leading_plus_enabled** | Option<**bool**> | This ensures the outbound origination attempts have a leading plus. Defaults to false to match conventional telecom behavior.  Usage: - Vonage/Twilio requires leading plus for all outbound calls. Set this to true.  @default false | [optional]
**tech_prefix** | Option<**String**> | This can be used to configure the tech prefix on outbound calls. This is an advanced property. | [optional]
**sip_diversion_header** | Option<**String**> | This can be used to enable the SIP diversion header for authenticating the calling number if the SIP trunk supports it. This is an advanced property. | [optional]
**sbc_configuration** | Option<[**serde_json::Value**](.md)> | This is an advanced configuration for enterprise deployments. This uses the onprem SBC to trunk into the SIP trunk's `gateways`, rather than the managed SBC provided by Vapi. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


