# Assistant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transcriber** | Option<[**models::CreateAssistantDtoTranscriber**](CreateAssistantDTO_transcriber.md)> |  | [optional]
**model** | Option<[**models::CreateAssistantDtoModel**](CreateAssistantDTO_model.md)> |  | [optional]
**voice** | Option<[**models::CreateAssistantDtoVoice**](CreateAssistantDTO_voice.md)> |  | [optional]
**first_message** | Option<**String**> | This is the first message that the assistant will say. This can also be a URL to a containerized audio file (mp3, wav, etc.).  If unspecified, assistant will wait for user to speak and use the model to respond once they speak. | [optional]
**first_message_interruptions_enabled** | Option<**bool**> |  | [optional][default to false]
**first_message_mode** | Option<**String**> | This is the mode for the first message. Default is 'assistant-speaks-first'.  Use: - 'assistant-speaks-first' to have the assistant speak first. - 'assistant-waits-for-user' to have the assistant wait for the user to speak first. - 'assistant-speaks-first-with-model-generated-message' to have the assistant speak first with a message generated by the model based on the conversation state. (`assistant.model.messages` at call start, `call.messages` at squad transfer points).  @default 'assistant-speaks-first' | [optional]
**voicemail_detection** | Option<[**models::CreateAssistantDtoVoicemailDetection**](CreateAssistantDTO_voicemailDetection.md)> |  | [optional]
**client_messages** | Option<**Vec<String>**> | These are the messages that will be sent to your Client SDKs. Default is conversation-update,function-call,hang,model-output,speech-update,status-update,transfer-update,transcript,tool-calls,user-interrupted,voice-input,workflow.node.started. You can check the shape of the messages in ClientMessage schema. | [optional]
**server_messages** | Option<**Vec<String>**> | These are the messages that will be sent to your Server URL. Default is conversation-update,end-of-call-report,function-call,hang,speech-update,status-update,tool-calls,transfer-destination-request,user-interrupted. You can check the shape of the messages in ServerMessage schema. | [optional]
**silence_timeout_seconds** | Option<**f64**> | How many seconds of silence to wait before ending the call. Defaults to 30.  @default 30 | [optional]
**max_duration_seconds** | Option<**f64**> | This is the maximum number of seconds that the call will last. When the call reaches this duration, it will be ended.  @default 600 (10 minutes) | [optional]
**background_sound** | Option<[**models::CreateAssistantDtoBackgroundSound**](CreateAssistantDTO_backgroundSound.md)> |  | [optional]
**background_denoising_enabled** | Option<**bool**> | This enables filtering of noise and background speech while the user is talking.  Default `false` while in beta.  @default false | [optional]
**model_output_in_messages_enabled** | Option<**bool**> | This determines whether the model's output is used in conversation history rather than the transcription of assistant's speech.  Default `false` while in beta.  @default false | [optional]
**transport_configurations** | Option<[**Vec<models::CreateAssistantDtoTransportConfigurationsInner>**](CreateAssistantDTO_transportConfigurations_inner.md)> | These are the configurations to be passed to the transport providers of assistant's calls, like Twilio. You can store multiple configurations for different transport providers. For a call, only the configuration matching the call transport provider is used. | [optional]
**observability_plan** | Option<[**models::LangfuseObservabilityPlan**](LangfuseObservabilityPlan.md)> | This is the plan for observability configuration of assistant's calls. Currently supports Langfuse for tracing and monitoring. | [optional]
**credentials** | Option<[**Vec<models::CreateAssistantDtoCredentialsInner>**](CreateAssistantDTO_credentials_inner.md)> | These are dynamic credentials that will be used for the assistant calls. By default, all the credentials are available for use in the call but you can supplement an additional credentials using this. Dynamic credentials override existing credentials. | [optional]
**name** | Option<**String**> | This is the name of the assistant.  This is required when you want to transfer between assistants in a call. | [optional]
**voicemail_message** | Option<**String**> | This is the message that the assistant will say if the call is forwarded to voicemail.  If unspecified, it will hang up. | [optional]
**end_call_message** | Option<**String**> | This is the message that the assistant will say if it ends the call.  If unspecified, it will hang up without saying anything. | [optional]
**end_call_phrases** | Option<**Vec<String>**> | This list contains phrases that, if spoken by the assistant, will trigger the call to be hung up. Case insensitive. | [optional]
**compliance_plan** | Option<[**models::CompliancePlan**](CompliancePlan.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | This is for metadata you want to store on the assistant. | [optional]
**analysis_plan** | Option<[**models::AnalysisPlan**](AnalysisPlan.md)> | This is the plan for analysis of assistant's calls. Stored in `call.analysis`. | [optional]
**artifact_plan** | Option<[**models::ArtifactPlan**](ArtifactPlan.md)> | This is the plan for artifacts generated during assistant's calls. Stored in `call.artifact`.  Note: `recordingEnabled` is currently at the root level. It will be moved to `artifactPlan` in the future, but will remain backwards compatible. | [optional]
**message_plan** | Option<[**models::MessagePlan**](MessagePlan.md)> | This is the plan for static predefined messages that can be spoken by the assistant during the call, like `idleMessages`.  Note: `firstMessage`, `voicemailMessage`, and `endCallMessage` are currently at the root level. They will be moved to `messagePlan` in the future, but will remain backwards compatible. | [optional]
**start_speaking_plan** | Option<[**models::StartSpeakingPlan**](StartSpeakingPlan.md)> | This is the plan for when the assistant should start talking.  You should configure this if you're running into these issues: - The assistant is too slow to start talking after the customer is done speaking. - The assistant is too fast to start talking after the customer is done speaking. - The assistant is so fast that it's actually interrupting the customer. | [optional]
**stop_speaking_plan** | Option<[**models::StopSpeakingPlan**](StopSpeakingPlan.md)> | This is the plan for when assistant should stop talking on customer interruption.  You should configure this if you're running into these issues: - The assistant is too slow to recognize customer's interruption. - The assistant is too fast to recognize customer's interruption. - The assistant is getting interrupted by phrases that are just acknowledgments. - The assistant is getting interrupted by background noises. - The assistant is not properly stopping -- it starts talking right after getting interrupted. | [optional]
**monitor_plan** | Option<[**models::MonitorPlan**](MonitorPlan.md)> | This is the plan for real-time monitoring of the assistant's calls.  Usage: - To enable live listening of the assistant's calls, set `monitorPlan.listenEnabled` to `true`. - To enable live control of the assistant's calls, set `monitorPlan.controlEnabled` to `true`.  Note, `serverMessages`, `clientMessages`, `serverUrl` and `serverUrlSecret` are currently at the root level but will be moved to `monitorPlan` in the future. Will remain backwards compatible | [optional]
**credential_ids** | Option<**Vec<String>**> | These are the credentials that will be used for the assistant calls. By default, all the credentials are available for use in the call but you can provide a subset using this. | [optional]
**server** | Option<[**models::Server**](Server.md)> | This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server.url 2. phoneNumber.serverUrl 3. org.serverUrl | [optional]
**hooks** | Option<[**Vec<models::AssistantHooks>**](AssistantHooks.md)> | This is a set of actions that will be performed on certain events. | [optional]
**keypad_input_plan** | Option<[**models::KeypadInputPlan**](KeypadInputPlan.md)> |  | [optional]
**id** | **String** | This is the unique identifier for the assistant. | 
**org_id** | **String** | This is the unique identifier for the org that this assistant belongs to. | 
**created_at** | **String** | This is the ISO 8601 date-time string of when the assistant was created. | 
**updated_at** | **String** | This is the ISO 8601 date-time string of when the assistant was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


