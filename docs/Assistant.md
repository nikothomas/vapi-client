# Assistant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transcriber** | Option<[**models::AssistantTranscriber**](AssistantTranscriber.md)> |  | [optional]
**model** | Option<[**models::AssistantModel**](AssistantModel.md)> |  | [optional]
**voice** | Option<[**models::AssistantVoice**](AssistantVoice.md)> |  | [optional]
**first_message** | Option<**String**> | This is the first message that the assistant will say. This can also be a URL to a containerized audio file (mp3, wav, etc.).  If unspecified, assistant will wait for user to speak and use the model to respond once they speak. | [optional]
**first_message_interruptions_enabled** | Option<**bool**> |  | [optional]
**first_message_mode** | Option<[**models::AssistantFirstMessageMode**](AssistantFirstMessageMode.md)> |  | [optional]
**voicemail_detection** | Option<[**models::AssistantVoicemailDetection**](AssistantVoicemailDetection.md)> |  | [optional]
**client_messages** | Option<[**Vec<models::AssistantClientMessagesItem>**](AssistantClientMessagesItem.md)> | These are the messages that will be sent to your Client SDKs. Default is conversation-update,function-call,hang,model-output,speech-update,status-update,transfer-update,transcript,tool-calls,user-interrupted,voice-input,workflow.node.started. You can check the shape of the messages in ClientMessage schema. | [optional]
**server_messages** | Option<[**Vec<models::AssistantServerMessagesItem>**](AssistantServerMessagesItem.md)> | These are the messages that will be sent to your Server URL. Default is conversation-update,end-of-call-report,function-call,hang,speech-update,status-update,tool-calls,transfer-destination-request,user-interrupted. You can check the shape of the messages in ServerMessage schema. | [optional]
**silence_timeout_seconds** | Option<**f64**> | How many seconds of silence to wait before ending the call. Defaults to 30.  @default 30 | [optional]
**max_duration_seconds** | Option<**f64**> | This is the maximum number of seconds that the call will last. When the call reaches this duration, it will be ended.  @default 600 (10 minutes) | [optional]
**background_sound** | Option<[**models::AssistantBackgroundSound**](AssistantBackgroundSound.md)> |  | [optional]
**background_denoising_enabled** | Option<**bool**> | This enables filtering of noise and background speech while the user is talking.  Default `false` while in beta.  @default false | [optional]
**model_output_in_messages_enabled** | Option<**bool**> | This determines whether the model's output is used in conversation history rather than the transcription of assistant's speech.  Default `false` while in beta.  @default false | [optional]
**transport_configurations** | Option<[**Vec<models::TransportConfigurationTwilio>**](TransportConfigurationTwilio.md)> | These are the configurations to be passed to the transport providers of assistant's calls, like Twilio. You can store multiple configurations for different transport providers. For a call, only the configuration matching the call transport provider is used. | [optional]
**observability_plan** | Option<[**models::LangfuseObservabilityPlan**](LangfuseObservabilityPlan.md)> |  | [optional]
**credentials** | Option<[**Vec<models::AssistantCredentialsItem>**](AssistantCredentialsItem.md)> | These are dynamic credentials that will be used for the assistant calls. By default, all the credentials are available for use in the call but you can supplement an additional credentials using this. Dynamic credentials override existing credentials. | [optional]
**name** | Option<**String**> | This is the name of the assistant.  This is required when you want to transfer between assistants in a call. | [optional]
**voicemail_message** | Option<**String**> | This is the message that the assistant will say if the call is forwarded to voicemail.  If unspecified, it will hang up. | [optional]
**end_call_message** | Option<**String**> | This is the message that the assistant will say if it ends the call.  If unspecified, it will hang up without saying anything. | [optional]
**end_call_phrases** | Option<**Vec<String>**> | This list contains phrases that, if spoken by the assistant, will trigger the call to be hung up. Case insensitive. | [optional]
**compliance_plan** | Option<[**models::CompliancePlan**](CompliancePlan.md)> |  | [optional]
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is for metadata you want to store on the assistant. | [optional]
**analysis_plan** | Option<[**models::AnalysisPlan**](AnalysisPlan.md)> |  | [optional]
**artifact_plan** | Option<[**models::ArtifactPlan**](ArtifactPlan.md)> |  | [optional]
**message_plan** | Option<[**models::MessagePlan**](MessagePlan.md)> |  | [optional]
**start_speaking_plan** | Option<[**models::StartSpeakingPlan**](StartSpeakingPlan.md)> |  | [optional]
**stop_speaking_plan** | Option<[**models::StopSpeakingPlan**](StopSpeakingPlan.md)> |  | [optional]
**monitor_plan** | Option<[**models::MonitorPlan**](MonitorPlan.md)> |  | [optional]
**credential_ids** | Option<**Vec<String>**> | These are the credentials that will be used for the assistant calls. By default, all the credentials are available for use in the call but you can provide a subset using this. | [optional]
**server** | Option<[**models::Server**](Server.md)> |  | [optional]
**hooks** | Option<[**Vec<models::AssistantHooks>**](AssistantHooks.md)> | This is a set of actions that will be performed on certain events. | [optional]
**keypad_input_plan** | Option<[**models::KeypadInputPlan**](KeypadInputPlan.md)> |  | [optional]
**id** | **String** | This is the unique identifier for the assistant. | 
**org_id** | **String** | This is the unique identifier for the org that this assistant belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the assistant was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


