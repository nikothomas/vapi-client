# ServerMessageResponseKnowledgeBaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**documents** | Option<[**Vec<models::KnowledgeBaseResponseDocument>**](KnowledgeBaseResponseDocument.md)> | This is the list of documents that will be sent to the model alongside the `messages` to generate a response. | [optional]
**message** | Option<[**models::CustomMessage**](CustomMessage.md)> | This can be used to skip the model output generation and speak a custom message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


