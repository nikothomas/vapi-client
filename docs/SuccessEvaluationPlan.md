# SuccessEvaluationPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rubric** | Option<[**models::SuccessEvaluationPlanRubric**](SuccessEvaluationPlanRubric.md)> |  | [optional]
**messages** | Option<[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)> | These are the messages used to generate the success evaluation.  @default: ``` [   {     \"role\": \"system\",     \"content\": \"You are an expert call evaluator. You will be given a transcript of a call and the system prompt of the AI participant. Determine if the call was successful based on the objectives inferred from the system prompt. DO NOT return anything except the result.\\n\\nRubric:\\\\n{{rubric}}\\n\\nOnly respond with the result.\"   },   {     \"role\": \"user\",     \"content\": \"Here is the transcript:\\n\\n{{transcript}}\\n\\n\"   },   {     \"role\": \"user\",     \"content\": \"Here was the system prompt of the call:\\n\\n{{systemPrompt}}\\n\\n\"   } ]```  You can customize by providing any messages you want.  Here are the template variables available: - {{transcript}}: the transcript of the call from `call.artifact.transcript`- {{systemPrompt}}: the system prompt of the call from `assistant.model.messages[type=system].content`- {{rubric}}: the rubric of the success evaluation from `successEvaluationPlan.rubric` | [optional]
**enabled** | Option<**bool**> | This determines whether a success evaluation is generated and stored in `call.analysis.successEvaluation`. Defaults to true.  Usage: - If you want to disable the success evaluation, set this to false.  @default true | [optional]
**timeout_seconds** | Option<**f64**> | This is how long the request is tried before giving up. When request times out, `call.analysis.successEvaluation` will be empty.  Usage: - To guarantee the success evaluation is generated, set this value high. Note, this will delay the end of call report in cases where model is slow to respond.  @default 5 seconds | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


