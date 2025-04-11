# CreateTransferCallToolDtoDestinationsItemOneOf3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<[**models::TransferDestinationSipMessage**](TransferDestinationSipMessage.md)> |  | [optional]
**sip_uri** | **String** | This is the SIP URI to transfer the call to. | 
**transfer_plan** | Option<[**models::TransferPlan**](TransferPlan.md)> |  | [optional]
**sip_headers** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | These are custom headers to be added to SIP refer during transfer call. | [optional]
**description** | Option<**String**> | This is the description of the destination, used by the AI to choose when and how to transfer the call. | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


