# SquadMemberDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant_id** | Option<**String**> | This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead. | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | This can be used to override the assistant's settings and provide values for it's template variables. | [optional]
**assistant_destinations** | Option<[**Vec<models::TransferDestinationAssistant>**](TransferDestinationAssistant.md)> | These are the others assistants that this assistant can transfer to.  If the assistant already has transfer call tool, these destinations are just appended to existing ones. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


