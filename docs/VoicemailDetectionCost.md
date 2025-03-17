# VoicemailDetectionCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the type of cost, always 'voicemail-detection' for this class. | 
**model** | [**serde_json::Value**](.md) | This is the model that was used to perform the analysis. | 
**provider** | **String** | This is the provider that was used to detect the voicemail. | 
**prompt_text_tokens** | **f64** | This is the number of prompt text tokens used in the voicemail detection. | 
**prompt_audio_tokens** | **f64** | This is the number of prompt audio tokens used in the voicemail detection. | 
**completion_text_tokens** | **f64** | This is the number of completion text tokens used in the voicemail detection. | 
**completion_audio_tokens** | **f64** | This is the number of completion audio tokens used in the voicemail detection. | 
**cost** | **f64** | This is the cost of the component in USD. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


