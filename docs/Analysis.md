# Analysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**summary** | Option<**String**> | This is the summary of the call. Customize by setting `assistant.analysisPlan.summaryPrompt`. | [optional]
**structured_data** | Option<[**serde_json::Value**](.md)> | This is the structured data extracted from the call. Customize by setting `assistant.analysisPlan.structuredDataPrompt` and/or `assistant.analysisPlan.structuredDataSchema`. | [optional]
**structured_data_multi** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | This is the structured data catalog of the call. Customize by setting `assistant.analysisPlan.structuredDataMultiPlan`. | [optional]
**success_evaluation** | Option<**String**> | This is the evaluation of the call. Customize by setting `assistant.analysisPlan.successEvaluationPrompt` and/or `assistant.analysisPlan.successEvaluationRubric`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


