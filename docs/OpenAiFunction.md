# OpenAiFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strict** | Option<**bool**> | This is a boolean that controls whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the parameters field. Only a subset of JSON Schema is supported when strict is true. Learn more about Structured Outputs in the [OpenAI guide](https://openai.com/index/introducing-structured-outputs-in-the-api/).  @default false | [optional][default to false]
**name** | **String** | This is the the name of the function to be called.  Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64. | 
**description** | Option<**String**> | This is the description of what the function does, used by the AI to choose when and how to call the function. | [optional]
**parameters** | Option<[**models::OpenAiFunctionParameters**](OpenAIFunctionParameters.md)> | These are the parameters the functions accepts, described as a JSON Schema object.  See the [OpenAI guide](https://platform.openai.com/docs/guides/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema) for documentation about the format.  Omitting parameters defines a function with an empty parameter list. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


