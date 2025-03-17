# UpdateTrieveKnowledgeBaseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | This is the name of the knowledge base. | [optional]
**search_plan** | Option<[**models::TrieveKnowledgeBaseSearchPlan**](TrieveKnowledgeBaseSearchPlan.md)> | This is the searching plan used when searching for relevant chunks from the vector store.  You should configure this if you're running into these issues: - Too much unnecessary context is being fed as knowledge base context. - Not enough relevant context is being fed as knowledge base context. | [optional]
**create_plan** | Option<[**models::TrieveKnowledgeBaseCreatePlan**](TrieveKnowledgeBase_createPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


