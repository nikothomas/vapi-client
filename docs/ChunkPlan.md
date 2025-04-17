# ChunkPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | This determines whether the model output is chunked before being sent to the voice provider. Default `true`.  Usage: - To rely on the voice provider's audio generation logic, set this to `false`. - If seeing issues with quality, set this to `true`.  If disabled, Vapi-provided audio control tokens like <flush /> will not work.  @default true | [optional]
**min_characters** | Option<**f64**> | This is the minimum number of characters in a chunk.  Usage: - To increase quality, set this to a higher value. - To decrease latency, set this to a lower value.  @default 30 | [optional]
**punctuation_boundaries** | Option<**Vec<String>**> | These are the punctuations that are considered valid boundaries for a chunk to be created.  Usage: - To increase quality, constrain to fewer boundaries. - To decrease latency, enable all.  Default is automatically set to balance the trade-off between quality and latency based on the provider. | [optional]
**format_plan** | Option<[**models::FormatPlan**](FormatPlan.md)> | This is the plan for formatting the chunk before it is sent to the voice provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


