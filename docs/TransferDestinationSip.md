# TransferDestinationSip

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<[**models::TransferDestinationAssistantMessage**](TransferDestinationAssistant_message.md)> |  | [optional]
**r#type** | **String** |  | 
**sip_uri** | **String** | This is the SIP URI to transfer the call to. | 
**transfer_plan** | Option<[**models::TransferPlan**](TransferPlan.md)> | This configures how transfer is executed and the experience of the destination party receiving the call. Defaults to `blind-transfer`.  @default `transferPlan.mode='blind-transfer'` | [optional]
**sip_headers** | Option<[**serde_json::Value**](.md)> | These are custom headers to be added to SIP refer during transfer call. | [optional]
**description** | Option<**String**> | This is the description of the destination, used by the AI to choose when and how to transfer the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


