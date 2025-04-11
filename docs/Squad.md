# Squad

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | This is the name of the squad. | [optional]
**members** | [**Vec<models::SquadMemberDto>**](SquadMemberDto.md) | This is the list of assistants that make up the squad.  The call will start with the first assistant in the list. | 
**members_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> |  | [optional]
**id** | **String** | This is the unique identifier for the squad. | 
**org_id** | **String** | This is the unique identifier for the org that this squad belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the squad was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the squad was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


