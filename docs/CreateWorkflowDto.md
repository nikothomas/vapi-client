# CreateWorkflowDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nodes** | [**Vec<models::WorkflowUserEditableNodesInner>**](WorkflowUserEditable_nodes_inner.md) |  | 
**transcriber** | Option<[**models::WorkflowUserEditableTranscriber**](WorkflowUserEditable_transcriber.md)> |  | [optional]
**voice** | Option<[**models::WorkflowUserEditableVoice**](WorkflowUserEditable_voice.md)> |  | [optional]
**observability_plan** | Option<[**models::LangfuseObservabilityPlan**](LangfuseObservabilityPlan.md)> | This is the plan for observability of workflow's calls.  Currently, only Langfuse is supported. | [optional]
**credentials** | Option<[**Vec<models::WorkflowUserEditableCredentialsInner>**](WorkflowUserEditable_credentials_inner.md)> | These are dynamic credentials that will be used for the workflow calls. By default, all the credentials are available for use in the call but you can supplement an additional credentials using this. Dynamic credentials override existing credentials. | [optional]
**name** | **String** |  | 
**edges** | [**Vec<models::Edge>**](Edge.md) |  | 
**global_prompt** | Option<**String**> |  | [optional]
**server** | Option<[**models::Server**](Server.md)> | This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. tool.server 2. workflow.server / assistant.server 3. phoneNumber.server 4. org.server | [optional]
**compliance_plan** | Option<[**models::CompliancePlan**](CompliancePlan.md)> | This is the compliance plan for the workflow. It allows you to configure HIPAA and other compliance settings. | [optional]
**analysis_plan** | Option<[**models::AnalysisPlan**](AnalysisPlan.md)> | This is the plan for analysis of workflow's calls. Stored in `call.analysis`. | [optional]
**artifact_plan** | Option<[**models::ArtifactPlan**](ArtifactPlan.md)> | This is the plan for artifacts generated during workflow's calls. Stored in `call.artifact`. | [optional]
**start_speaking_plan** | Option<[**models::StartSpeakingPlan**](StartSpeakingPlan.md)> | This is the plan for when the workflow nodes should start talking.  You should configure this if you're running into these issues: - The assistant is too slow to start talking after the customer is done speaking. - The assistant is too fast to start talking after the customer is done speaking. - The assistant is so fast that it's actually interrupting the customer. | [optional]
**stop_speaking_plan** | Option<[**models::StopSpeakingPlan**](StopSpeakingPlan.md)> | This is the plan for when workflow nodes should stop talking on customer interruption.  You should configure this if you're running into these issues: - The assistant is too slow to recognize customer's interruption. - The assistant is too fast to recognize customer's interruption. - The assistant is getting interrupted by phrases that are just acknowledgments. - The assistant is getting interrupted by background noises. - The assistant is not properly stopping -- it starts talking right after getting interrupted. | [optional]
**monitor_plan** | Option<[**models::MonitorPlan**](MonitorPlan.md)> | This is the plan for real-time monitoring of the workflow's calls.  Usage: - To enable live listening of the workflow's calls, set `monitorPlan.listenEnabled` to `true`. - To enable live control of the workflow's calls, set `monitorPlan.controlEnabled` to `true`. | [optional]
**background_speech_denoising_plan** | Option<[**models::BackgroundSpeechDenoisingPlan**](BackgroundSpeechDenoisingPlan.md)> | This enables filtering of noise and background speech while the user is talking.  Features: - Smart denoising using Krisp - Fourier denoising  Both can be used together. Order of precedence: - Smart denoising - Fourier denoising | [optional]
**credential_ids** | Option<**Vec<String>**> | These are the credentials that will be used for the workflow calls. By default, all the credentials are available for use in the call but you can provide a subset using this. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


