# WorkflowOpenAiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the provider of the model (`openai`). | 
**model** | **String** | This is the OpenAI model that will be used.  When using Vapi OpenAI or your own Azure Credentials, you have the option to specify the region for the selected model. This shouldn't be specified unless you have a specific reason to do so. Vapi will automatically find the fastest region that make sense. This is helpful when you are required to comply with Data Residency rules. Learn more about Azure regions here https://azure.microsoft.com/en-us/explore/global-infrastructure/data-residency/. | 
**temperature** | Option<**f64**> | This is the temperature of the model. | [optional]
**max_tokens** | Option<**f64**> | This is the max tokens of the model. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


