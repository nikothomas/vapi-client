# StructuredDataPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | Option<[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)> | These are the messages used to generate the structured data.  @default: ``` [   {     \"role\": \"system\",     \"content\": \"You are an expert data extractor. You will be given a transcript of a call. Extract structured data per the JSON Schema. DO NOT return anything except the structured data.\\n\\nJson Schema:\\\\n{{schema}}\\n\\nOnly respond with the JSON.\"   },   {     \"role\": \"user\",     \"content\": \"Here is the transcript:\\n\\n{{transcript}}\\n\\n\"   } ]```  You can customize by providing any messages you want.  Here are the template variables available: - {{transcript}}: the transcript of the call from `call.artifact.transcript`- {{systemPrompt}}: the system prompt of the call from `assistant.model.messages[type=system].content`- {{schema}}: the schema of the structured data from `structuredDataPlan.schema` | [optional]
**enabled** | Option<**bool**> | This determines whether structured data is generated and stored in `call.analysis.structuredData`. Defaults to false.  Usage: - If you want to extract structured data, set this to true and provide a `schema`.  @default false | [optional]
**schema** | Option<[**models::JsonSchema**](JsonSchema.md)> |  | [optional]
**timeout_seconds** | Option<**f64**> | This is how long the request is tried before giving up. When request times out, `call.analysis.structuredData` will be empty.  Usage: - To guarantee the structured data is generated, set this value high. Note, this will delay the end of call report in cases where model is slow to respond.  @default 5 seconds | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


