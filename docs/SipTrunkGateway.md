# SipTrunkGateway

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip** | **String** | This is the address of the gateway. It can be an IPv4 address like 1.1.1.1 or a fully qualified domain name like my-sip-trunk.pstn.twilio.com. | 
**port** | Option<**f64**> | This is the port number of the gateway. Default is 5060.  @default 5060 | [optional]
**netmask** | Option<**f64**> | This is the netmask of the gateway. Defaults to 32.  @default 32 | [optional]
**inbound_enabled** | Option<**bool**> | This is whether inbound calls are allowed from this gateway. Default is true.  @default true | [optional]
**outbound_enabled** | Option<**bool**> | This is whether outbound calls should be sent to this gateway. Default is true.  Note, if netmask is less than 32, it doesn't affect the outbound IPs that are tried. 1 attempt is made to `ip:port`.  @default true | [optional]
**outbound_protocol** | Option<**String**> | This is the protocol to use for SIP signaling outbound calls. Default is udp.  @default udp | [optional]
**options_ping_enabled** | Option<**bool**> | This is whether to send options ping to the gateway. This can be used to check if the gateway is reachable. Default is false.  This is useful for high availability setups where you want to check if the gateway is reachable before routing calls to it. Note, if no gateway for a trunk is reachable, outbound calls will be rejected.  @default false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


