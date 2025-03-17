# WebhookCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** |  | 
**authentication_plan** | [**models::OAuth2AuthenticationPlan**](OAuth2AuthenticationPlan.md) | This is the authentication plan. Currently supports OAuth2 RFC 6749. | 
**id** | **String** | This is the unique identifier for the credential. | 
**org_id** | **String** | This is the unique identifier for the org that this credential belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the credential was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 
**authentication_session** | [**models::Oauth2AuthenticationSession**](Oauth2AuthenticationSession.md) | This is the authentication session for the credential. Available for credentials that have an authentication plan. | 
**name** | Option<**String**> | This is the name of credential. This is just for your reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


