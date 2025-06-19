# StartSpeakingPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wait_seconds** | Option<**f64**> | This is how long assistant waits before speaking. Defaults to 0.4.  This is the minimum it will wait but if there is latency is the pipeline, this minimum will be exceeded. This is intended as a stopgap in case the pipeline is moving too fast.  Example: - If model generates tokens and voice generates bytes within 100ms, the pipeline still waits 300ms before outputting speech.  Usage: - If the customer is taking long pauses, set this to a higher value. - If the assistant is accidentally jumping in too much, set this to a higher value.  @default 0.4 | [optional]
**smart_endpointing_enabled** | Option<[**models::StartSpeakingPlanSmartEndpointingEnabled**](StartSpeakingPlan_smartEndpointingEnabled.md)> |  | [optional]
**smart_endpointing_plan** | Option<[**models::StartSpeakingPlanSmartEndpointingPlan**](StartSpeakingPlan_smartEndpointingPlan.md)> |  | [optional]
**custom_endpointing_rules** | Option<[**Vec<models::StartSpeakingPlanCustomEndpointingRulesInner>**](StartSpeakingPlan_customEndpointingRules_inner.md)> | These are the custom endpointing rules to set an endpointing timeout based on a regex on the customer's speech or the assistant's last message.  Usage: - If you have yes/no questions like \"are you interested in a loan?\", you can set a shorter timeout. - If you have questions where the customer may pause to look up information like \"what's my account number?\", you can set a longer timeout. - If you want to wait longer while customer is enumerating a list of numbers, you can set a longer timeout.  These rules have the highest precedence and will override both `smartEndpointingPlan` and `transcriptionEndpointingPlan` when a rule is matched.  The rules are evaluated in order and the first one that matches will be used.  Order of precedence for endpointing: 1. customEndpointingRules (if any match) 2. smartEndpointingPlan (if set) 3. transcriptionEndpointingPlan  @default [] | [optional]
**transcription_endpointing_plan** | Option<[**models::TranscriptionEndpointingPlan**](TranscriptionEndpointingPlan.md)> | This determines how a customer speech is considered done (endpointing) using the transcription of customer's speech.  Once an endpoint is triggered, the request is sent to `assistant.model`.  Note: This plan is only used if `smartEndpointingPlan` is not set. If both are provided, `smartEndpointingPlan` takes precedence. This plan will also be overridden by any matching `customEndpointingRules`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


