# TesterPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | Pass a transient assistant to use for the test assistant.  Make sure to write a detailed system prompt for a test assistant, and use the {{test.script}} variable to access the test script. | [optional]
**assistant_id** | Option<**String**> | Pass an assistant id that can be access  Make sure to write a detailed system prompt for the test assistant, and use the {{test.script}} variable to access the test script. | [optional]
**assistant_overrides** | Option<[**models::AssistantOverrides**](AssistantOverrides.md)> | Add any assistant overrides to the test assistant.  One use case is if you want to pass custom variables into the test using variableValues, that you can then access in the script and rubric using {{varName}}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


