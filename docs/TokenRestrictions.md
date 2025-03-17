# TokenRestrictions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | This determines whether the token is enabled or disabled. Default is true, it's enabled. | [optional]
**allowed_origins** | Option<**Vec<String>**> | This determines the allowed origins for this token. Validates the `Origin` header. Default is any origin.  Only relevant for `public` tokens. | [optional]
**allowed_assistant_ids** | Option<**Vec<String>**> | This determines which assistantIds can be used when creating a call. Default is any assistantId.  Only relevant for `public` tokens. | [optional]
**allow_transient_assistant** | Option<**bool**> | This determines whether transient assistants can be used when creating a call. Default is true.  If `allowedAssistantIds` is provided, this is automatically false.  Only relevant for `public` tokens. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


