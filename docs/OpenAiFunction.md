# OpenAiFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strict** | Option<**bool**> | This is a boolean that controls whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the parameters field. Only a subset of JSON Schema is supported when strict is true. Learn more about Structured Outputs in the [OpenAI guide](https://openai.com/index/introducing-structured-outputs-in-the-api/).  @default false | [optional]
**name** | **String** | This is the the name of the function to be called.  Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64. | 
**description** | Option<**String**> | This is the description of what the function does, used by the AI to choose when and how to call the function. | [optional]
**parameters** | Option<[**models::OpenAiFunctionParameters**](OpenAiFunctionParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


