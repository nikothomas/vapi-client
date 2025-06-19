# CostBreakdown

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transport** | Option<**f64**> | This is the cost of the transport provider, like Twilio or Vonage. | [optional]
**stt** | Option<**f64**> | This is the cost of the speech-to-text service. | [optional]
**llm** | Option<**f64**> | This is the cost of the language model. | [optional]
**tts** | Option<**f64**> | This is the cost of the text-to-speech service. | [optional]
**vapi** | Option<**f64**> | This is the cost of Vapi. | [optional]
**chat** | Option<**f64**> | This is the cost of chat interactions. | [optional]
**total** | Option<**f64**> | This is the total cost of the call. | [optional]
**llm_prompt_tokens** | Option<**f64**> | This is the LLM prompt tokens used for the call. | [optional]
**llm_completion_tokens** | Option<**f64**> | This is the LLM completion tokens used for the call. | [optional]
**tts_characters** | Option<**f64**> | This is the TTS characters used for the call. | [optional]
**analysis_cost_breakdown** | Option<[**models::AnalysisCostBreakdown**](AnalysisCostBreakdown.md)> | This is the cost of the analysis. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


