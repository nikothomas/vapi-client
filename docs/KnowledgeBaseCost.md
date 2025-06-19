# KnowledgeBaseCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of cost, always 'knowledge-base' for this class. | 
**model** | [**serde_json::Value**](.md) | This is the model that was used for processing the knowledge base. | 
**prompt_tokens** | **f64** | This is the number of prompt tokens used in the knowledge base query. | 
**completion_tokens** | **f64** | This is the number of completion tokens generated in the knowledge base query. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


