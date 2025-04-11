# AnalysisPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**summary_plan** | Option<[**models::SummaryPlan**](SummaryPlan.md)> |  | [optional]
**structured_data_plan** | Option<[**models::StructuredDataPlan**](StructuredDataPlan.md)> |  | [optional]
**structured_data_multi_plan** | Option<[**Vec<models::StructuredDataMultiPlan>**](StructuredDataMultiPlan.md)> | This is an array of structured data plan catalogs. Each entry includes a `key` and a `plan` for generating the structured data from the call. This outputs to `call.analysis.structuredDataMulti`. | [optional]
**success_evaluation_plan** | Option<[**models::SuccessEvaluationPlan**](SuccessEvaluationPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


