# WorkflowAnthropicModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** | This is the provider of the model (`anthropic`). | 
**model** | **String** | This is the specific model that will be used. | 
**thinking** | Option<[**models::AnthropicThinkingConfig**](AnthropicThinkingConfig.md)> | This is the optional configuration for Anthropic's thinking feature.  - Only applicable for `claude-3-7-sonnet-20250219` model. - If provided, `maxTokens` must be greater than `thinking.budgetTokens`. | [optional]
**temperature** | Option<**f64**> | This is the temperature of the model. | [optional]
**max_tokens** | Option<**f64**> | This is the max tokens of the model. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


