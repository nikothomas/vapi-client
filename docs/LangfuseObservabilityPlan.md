# LangfuseObservabilityPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | **String** |  | 
**tags** | **Vec<String>** | This is an array of tags to be added to the Langfuse trace. Tags allow you to categorize and filter traces. https://langfuse.com/docs/tracing-features/tags | 
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is a JSON object that will be added to the Langfuse trace. Traces can be enriched with metadata to better understand your users, application, and experiments. https://langfuse.com/docs/tracing-features/metadata By default it includes the call metadata, assistant metadata, and assistant overrides. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


