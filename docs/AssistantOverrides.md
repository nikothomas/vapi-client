# AssistantOverrides

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transcriber** | Option<[**models::AssistantOverridesTranscriber**](AssistantOverridesTranscriber.md)> |  | [optional]
**model** | Option<[**models::AssistantOverridesModel**](AssistantOverridesModel.md)> |  | [optional]
**voice** | Option<[**models::AssistantOverridesVoice**](AssistantOverridesVoice.md)> |  | [optional]
**first_message** | Option<**String**> | This is the first message that the assistant will say. This can also be a URL to a containerized audio file (mp3, wav, etc.).  If unspecified, assistant will wait for user to speak and use the model to respond once they speak. | [optional]
**first_message_interruptions_enabled** | Option<**bool**> |  | [optional]
**first_message_mode** | Option<[**models::AssistantOverridesFirstMessageMode**](AssistantOverridesFirstMessageMode.md)> |  | [optional]
**voicemail_detection** | Option<[**models::AssistantOverridesVoicemailDetection**](AssistantOverridesVoicemailDetection.md)> |  | [optional]
**client_messages** | Option<[**Vec<models::AssistantOverridesClientMessagesItem>**](AssistantOverridesClientMessagesItem.md)> | These are the messages that will be sent to your Client SDKs. Default is conversation-update,function-call,hang,model-output,speech-update,status-update,transfer-update,transcript,tool-calls,user-interrupted,voice-input,workflow.node.started. You can check the shape of the messages in ClientMessage schema. | [optional]
**server_messages** | Option<[**Vec<models::AssistantOverridesServerMessagesItem>**](AssistantOverridesServerMessagesItem.md)> | These are the messages that will be sent to your Server URL. Default is conversation-update,end-of-call-report,function-call,hang,speech-update,status-update,tool-calls,transfer-destination-request,user-interrupted. You can check the shape of the messages in ServerMessage schema. | [optional]
**silence_timeout_seconds** | Option<**f64**> | How many seconds of silence to wait before ending the call. Defaults to 30.  @default 30 | [optional]
**max_duration_seconds** | Option<**f64**> | This is the maximum number of seconds that the call will last. When the call reaches this duration, it will be ended.  @default 600 (10 minutes) | [optional]
**background_sound** | Option<[**models::AssistantOverridesBackgroundSound**](AssistantOverridesBackgroundSound.md)> |  | [optional]
**background_denoising_enabled** | Option<**bool**> | This enables filtering of noise and background speech while the user is talking.  Default `false` while in beta.  @default false | [optional]
**model_output_in_messages_enabled** | Option<**bool**> | This determines whether the model's output is used in conversation history rather than the transcription of assistant's speech.  Default `false` while in beta.  @default false | [optional]
**transport_configurations** | Option<[**Vec<models::TransportConfigurationTwilio>**](TransportConfigurationTwilio.md)> | These are the configurations to be passed to the transport providers of assistant's calls, like Twilio. You can store multiple configurations for different transport providers. For a call, only the configuration matching the call transport provider is used. | [optional]
**observability_plan** | Option<[**models::LangfuseObservabilityPlan**](LangfuseObservabilityPlan.md)> |  | [optional]
**credentials** | Option<[**Vec<models::AssistantOverridesCredentialsItem>**](AssistantOverridesCredentialsItem.md)> | These are dynamic credentials that will be used for the assistant calls. By default, all the credentials are available for use in the call but you can supplement an additional credentials using this. Dynamic credentials override existing credentials. | [optional]
**variable_values** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | These are values that will be used to replace the template variables in the assistant messages and other text-based fields. This uses LiquidJS syntax. https://liquidjs.com/tutorials/intro-to-liquid.html  So for example, `{{ name }}` will be replaced with the value of `name` in `variableValues`. `{{\"now\" | date: \"%b %d, %Y, %I:%M %p\", \"America/New_York\"}}` will be replaced with the current date and time in New York.  Some VAPI reserved defaults:  - *customer* - the customer object | [optional]
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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


