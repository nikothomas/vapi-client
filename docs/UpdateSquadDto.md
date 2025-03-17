# UpdateSquadDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | This is the name of the squad. | [optional]
**members** | [**Vec<models::SquadMemberDto>**](SquadMemberDTO.md) | This is the list of assistants that make up the squad.  The call will start with the first assistant in the list. | 
**members_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | This can be used to override all the assistants' settings and provide values for their template variables.  Both `membersOverrides` and `members[n].assistantOverrides` can be used together. First, `members[n].assistantOverrides` is applied. Then, `membersOverrides` is applied as a global override. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


