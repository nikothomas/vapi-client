# ServerMessageResponseAssistantRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<[**models::ServerMessageResponseAssistantRequestDestination**](ServerMessageResponseAssistantRequestDestination.md)> |  | [optional]
**assistant_id** | Option<**String**> | This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDto.md)> |  | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> |  | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for the call. To use a transient squad, use `squad` instead. | [optional]
**squad** | Option<[**models::CreateSquadDto**](CreateSquadDto.md)> |  | [optional]
**error** | Option<**String**> | This is the error if the call shouldn't be accepted. This is spoken to the customer.  If this is sent, `assistantId`, `assistant`, `squadId`, `squad`, and `destination` are ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


