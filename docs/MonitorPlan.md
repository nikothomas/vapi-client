# MonitorPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**listen_enabled** | Option<**bool**> | This determines whether the assistant's calls allow live listening. Defaults to true.  Fetch `call.monitor.listenUrl` to get the live listening URL.  @default true | [optional]
**control_enabled** | Option<**bool**> | This determines whether the assistant's calls allow live control. Defaults to true.  Fetch `call.monitor.controlUrl` to get the live control URL.  To use, send any control message via a POST request to `call.monitor.controlUrl`. Here are the types of controls supported: https://docs.vapi.ai/api-reference/messages/client-inbound-message  @default true | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


