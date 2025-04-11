# VoiceLibrary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is the voice provider that will be used. | [optional]
**provider_id** | Option<**String**> | The ID of the voice provided by the provider. | [optional]
**slug** | Option<**String**> | The unique slug of the voice. | [optional]
**name** | Option<**String**> | The name of the voice. | [optional]
**language** | Option<**String**> | The language of the voice. | [optional]
**language_code** | Option<**String**> | The language code of the voice. | [optional]
**model** | Option<**String**> | The model of the voice. | [optional]
**supported_models** | Option<**String**> | The supported models of the voice. | [optional]
**gender** | Option<[**models::VoiceLibraryGender**](VoiceLibraryGender.md)> |  | [optional]
**accent** | Option<**String**> | The accent of the voice. | [optional]
**preview_url** | Option<**String**> | The preview URL of the voice. | [optional]
**description** | Option<**String**> | The description of the voice. | [optional]
**credential_id** | Option<**String**> | The credential ID of the voice. | [optional]
**id** | **String** | The unique identifier for the voice library. | 
**org_id** | **String** | The unique identifier for the organization that this voice library belongs to. | 
**is_public** | **bool** | The Public voice is shared accross all the organizations. | 
**is_deleted** | **bool** | The deletion status of the voice. | 
**created_at** | **String** | The ISO 8601 date-time string of when the voice library was created. | 
**updated_at** | **String** | The ISO 8601 date-time string of when the voice library was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


