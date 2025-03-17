# CreateWebCallDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant_id** | Option<**String**> | This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead. | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | These are the overrides for the `assistant` or `assistantId`'s settings and template variables. | [optional]
**squad_id** | Option<**String**> | This is the squad that will be used for the call. To use a transient squad, use `squad` instead. | [optional]
**squad** | Option<[**models::CreateSquadDto**](CreateSquadDTO.md)> | This is a squad that will be used for the call. To use an existing squad, use `squadId` instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


