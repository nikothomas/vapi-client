# AnalysisCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of cost, always 'analysis' for this class. | 
**analysis_type** | **String** | This is the type of analysis performed. | 
**model** | [**serde_json::Value**](.md) | This is the model that was used to perform the analysis. | 
**prompt_tokens** | **f64** | This is the number of prompt tokens used in the analysis. | 
**completion_tokens** | **f64** | This is the number of completion tokens generated in the analysis. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


