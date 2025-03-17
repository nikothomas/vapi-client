# Template

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**details** | Option<[**models::CreateToolTemplateDtoDetails**](CreateToolTemplateDTO_details.md)> |  | [optional]
**provider_details** | Option<[**models::CreateToolTemplateDtoProviderDetails**](CreateToolTemplateDTO_providerDetails.md)> |  | [optional]
**metadata** | Option<[**models::ToolTemplateMetadata**](ToolTemplateMetadata.md)> |  | [optional]
**visibility** | Option<**String**> |  | [optional][default to Private]
**r#type** | **String** |  | [default to Tool]
**name** | Option<**String**> | The name of the template. This is just for your own reference. | [optional]
**provider** | Option<**String**> |  | [optional]
**id** | **String** | The unique identifier for the template. | 
**org_id** | **String** | The unique identifier for the organization that this template belongs to. | 
**created_at** | **String** | The ISO 8601 date-time string of when the template was created. | 
**updated_at** | **String** | The ISO 8601 date-time string of when the template was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


