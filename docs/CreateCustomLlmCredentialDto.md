# CreateCustomLlmCredentialDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** |  | 
**api_key** | **String** | This is not returned in the API. | 
**authentication_plan** | Option<[**models::OAuth2AuthenticationPlan**](OAuth2AuthenticationPlan.md)> | This is the authentication plan. Currently supports OAuth2 RFC 6749. To use Bearer authentication, use apiKey | [optional]
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


