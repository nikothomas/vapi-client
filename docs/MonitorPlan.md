# MonitorPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**listen_enabled** | Option<**bool**> | This determines whether the assistant's calls allow live listening. Defaults to true.  Fetch `call.monitor.listenUrl` to get the live listening URL.  @default true | [optional]
**listen_authentication_enabled** | Option<**bool**> | This enables authentication on the `call.monitor.listenUrl`.  If `listenAuthenticationEnabled` is `true`, the `call.monitor.listenUrl` will require an `Authorization: Bearer <vapi-public-api-key>` header.  @default false | [optional]
**control_enabled** | Option<**bool**> | This determines whether the assistant's calls allow live control. Defaults to true.  Fetch `call.monitor.controlUrl` to get the live control URL.  To use, send any control message via a POST request to `call.monitor.controlUrl`. Here are the types of controls supported: https://docs.vapi.ai/api-reference/messages/client-inbound-message  @default true | [optional]
**control_authentication_enabled** | Option<**bool**> | This enables authentication on the `call.monitor.controlUrl`.  If `controlAuthenticationEnabled` is `true`, the `call.monitor.controlUrl` will require an `Authorization: Bearer <vapi-public-api-key>` header.  @default false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


