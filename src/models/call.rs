/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct Call {
    /// This is the type of call.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// These are the costs of individual components of the call in USD.
    #[serde(rename = "costs", skip_serializing_if = "Option::is_none")]
    pub costs: Option<Vec<models::CallCostsInner>>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::ArtifactMessagesInner>>,
    /// This is the provider of the call.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "phoneCallProvider", skip_serializing_if = "Option::is_none")]
    pub phone_call_provider: Option<PhoneCallProvider>,
    /// This is the transport of the phone call.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "phoneCallTransport", skip_serializing_if = "Option::is_none")]
    pub phone_call_transport: Option<PhoneCallTransport>,
    /// This is the status of the call.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// This is the explanation for how the call ended.
    #[serde(rename = "endedReason", skip_serializing_if = "Option::is_none")]
    pub ended_reason: Option<EndedReason>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<models::CallDestination>,
    /// This is the unique identifier for the call.
    #[serde(rename = "id")]
    pub id: String,
    /// This is the unique identifier for the org that this call belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the ISO 8601 date-time string of when the call was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// This is the ISO 8601 date-time string of when the call was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// This is the ISO 8601 date-time string of when the call was started.
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// This is the ISO 8601 date-time string of when the call was ended.
    #[serde(rename = "endedAt", skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// This is the cost of the call in USD.
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    /// This is the cost of the call in USD.
    #[serde(rename = "costBreakdown", skip_serializing_if = "Option::is_none")]
    pub cost_breakdown: Option<models::CostBreakdown>,
    /// This is a copy of assistant artifact plan. This isn't actually stored on the call but rather just returned in POST /call/web to enable artifact creation client side.
    #[serde(rename = "artifactPlan", skip_serializing_if = "Option::is_none")]
    pub artifact_plan: Option<models::ArtifactPlan>,
    /// This is the analysis of the call. Configure in `assistant.analysisPlan`.
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<models::Analysis>,
    /// This is to real-time monitor the call. Configure in `assistant.monitorPlan`.
    #[serde(rename = "monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: Option<models::Monitor>,
    /// These are the artifacts created from the call. Configure in `assistant.artifactPlan`.
    #[serde(rename = "artifact", skip_serializing_if = "Option::is_none")]
    pub artifact: Option<models::Artifact>,
    /// This is the transport used for the call.
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<models::Transport>,
    /// The ID of the call as provided by the phone number service. callSid in Twilio. conversationUuid in Vonage.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(
        rename = "phoneCallProviderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_call_provider_id: Option<String>,
    /// This is the assistant that will be used for the call. To use a transient assistant, use `assistant` instead.
    #[serde(rename = "assistantId", skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// This is the assistant that will be used for the call. To use an existing assistant, use `assistantId` instead.
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    /// These are the overrides for the `assistant` or `assistantId`'s settings and template variables.
    #[serde(rename = "assistantOverrides", skip_serializing_if = "Option::is_none")]
    pub assistant_overrides: Option<models::AssistantOverrides>,
    /// This is the squad that will be used for the call. To use a transient squad, use `squad` instead.
    #[serde(rename = "squadId", skip_serializing_if = "Option::is_none")]
    pub squad_id: Option<String>,
    /// This is a squad that will be used for the call. To use an existing squad, use `squadId` instead.
    #[serde(rename = "squad", skip_serializing_if = "Option::is_none")]
    pub squad: Option<models::CreateSquadDto>,
    /// This is the phone number that will be used for the call. To use a transient number, use `phoneNumber` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "phoneNumberId", skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    /// This is the phone number that will be used for the call. To use an existing number, use `phoneNumberId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ImportTwilioPhoneNumberDto>,
    /// This is the customer that will be called. To call a transient customer , use `customer` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// This is the customer that will be called. To call an existing customer, use `customerId` instead.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<models::CreateCustomerDto>,
    /// This is the name of the call. This is just for your own reference.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Call {
    pub fn new(id: String, org_id: String, created_at: String, updated_at: String) -> Call {
        Call {
            r#type: None,
            costs: None,
            messages: None,
            phone_call_provider: None,
            phone_call_transport: None,
            status: None,
            ended_reason: None,
            destination: None,
            id,
            org_id,
            created_at,
            updated_at,
            started_at: None,
            ended_at: None,
            cost: None,
            cost_breakdown: None,
            artifact_plan: None,
            analysis: None,
            monitor: None,
            artifact: None,
            transport: None,
            phone_call_provider_id: None,
            assistant_id: None,
            assistant: None,
            assistant_overrides: None,
            squad_id: None,
            squad: None,
            phone_number_id: None,
            phone_number: None,
            customer_id: None,
            customer: None,
            name: None,
        }
    }
}
/// This is the type of call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "inboundPhoneCall")]
    InboundPhoneCall,
    #[serde(rename = "outboundPhoneCall")]
    OutboundPhoneCall,
    #[serde(rename = "webCall")]
    WebCall,
}

impl Default for Type {
    fn default() -> Type {
        Self::InboundPhoneCall
    }
}
/// This is the provider of the call.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum PhoneCallProvider {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "vonage")]
    Vonage,
    #[serde(rename = "vapi")]
    Vapi,
}

impl Default for PhoneCallProvider {
    fn default() -> PhoneCallProvider {
        Self::Twilio
    }
}
/// This is the transport of the phone call.  Only relevant for `outboundPhoneCall` and `inboundPhoneCall` type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum PhoneCallTransport {
    #[serde(rename = "sip")]
    Sip,
    #[serde(rename = "pstn")]
    Pstn,
}

impl Default for PhoneCallTransport {
    fn default() -> PhoneCallTransport {
        Self::Sip
    }
}
/// This is the status of the call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "forwarding")]
    Forwarding,
    #[serde(rename = "ended")]
    Ended,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}
/// This is the explanation for how the call ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum EndedReason {
    #[serde(rename = "assistant-not-valid")]
    AssistantNotValid,
    #[serde(rename = "assistant-not-provided")]
    AssistantNotProvided,
    #[serde(rename = "call-start-error-neither-assistant-nor-server-set")]
    CallStartErrorNeitherAssistantNorServerSet,
    #[serde(rename = "assistant-request-failed")]
    AssistantRequestFailed,
    #[serde(rename = "assistant-request-returned-error")]
    AssistantRequestReturnedError,
    #[serde(rename = "assistant-request-returned-unspeakable-error")]
    AssistantRequestReturnedUnspeakableError,
    #[serde(rename = "assistant-request-returned-invalid-assistant")]
    AssistantRequestReturnedInvalidAssistant,
    #[serde(rename = "assistant-request-returned-no-assistant")]
    AssistantRequestReturnedNoAssistant,
    #[serde(rename = "assistant-request-returned-forwarding-phone-number")]
    AssistantRequestReturnedForwardingPhoneNumber,
    #[serde(rename = "assistant-ended-call")]
    AssistantEndedCall,
    #[serde(rename = "assistant-said-end-call-phrase")]
    AssistantSaidEndCallPhrase,
    #[serde(rename = "assistant-ended-call-with-hangup-task")]
    AssistantEndedCallWithHangupTask,
    #[serde(rename = "assistant-forwarded-call")]
    AssistantForwardedCall,
    #[serde(rename = "assistant-join-timed-out")]
    AssistantJoinTimedOut,
    #[serde(rename = "customer-busy")]
    CustomerBusy,
    #[serde(rename = "customer-ended-call")]
    CustomerEndedCall,
    #[serde(rename = "customer-did-not-answer")]
    CustomerDidNotAnswer,
    #[serde(rename = "customer-did-not-give-microphone-permission")]
    CustomerDidNotGiveMicrophonePermission,
    #[serde(rename = "assistant-said-message-with-end-call-enabled")]
    AssistantSaidMessageWithEndCallEnabled,
    #[serde(rename = "exceeded-max-duration")]
    ExceededMaxDuration,
    #[serde(rename = "manually-canceled")]
    ManuallyCanceled,
    #[serde(rename = "phone-call-provider-closed-websocket")]
    PhoneCallProviderClosedWebsocket,
    #[serde(rename = "db-error")]
    DbError,
    #[serde(rename = "assistant-not-found")]
    AssistantNotFound,
    #[serde(rename = "license-check-failed")]
    LicenseCheckFailed,
    #[serde(rename = "pipeline-error-openai-voice-failed")]
    PipelineErrorOpenaiVoiceFailed,
    #[serde(rename = "pipeline-error-cartesia-voice-failed")]
    PipelineErrorCartesiaVoiceFailed,
    #[serde(rename = "pipeline-error-deepgram-voice-failed")]
    PipelineErrorDeepgramVoiceFailed,
    #[serde(rename = "pipeline-error-eleven-labs-voice-failed")]
    PipelineErrorElevenLabsVoiceFailed,
    #[serde(rename = "pipeline-error-playht-voice-failed")]
    PipelineErrorPlayhtVoiceFailed,
    #[serde(rename = "pipeline-error-lmnt-voice-failed")]
    PipelineErrorLmntVoiceFailed,
    #[serde(rename = "pipeline-error-azure-voice-failed")]
    PipelineErrorAzureVoiceFailed,
    #[serde(rename = "pipeline-error-rime-ai-voice-failed")]
    PipelineErrorRimeAiVoiceFailed,
    #[serde(rename = "pipeline-error-neets-voice-failed")]
    PipelineErrorNeetsVoiceFailed,
    #[serde(rename = "pipeline-error-smallest-ai-voice-failed")]
    PipelineErrorSmallestAiVoiceFailed,
    #[serde(rename = "pipeline-error-neuphonic-voice-failed")]
    PipelineErrorNeuphonicVoiceFailed,
    #[serde(rename = "pipeline-error-hume-voice-failed")]
    PipelineErrorHumeVoiceFailed,
    #[serde(rename = "pipeline-error-deepgram-transcriber-failed")]
    PipelineErrorDeepgramTranscriberFailed,
    #[serde(rename = "pipeline-error-gladia-transcriber-failed")]
    PipelineErrorGladiaTranscriberFailed,
    #[serde(rename = "pipeline-error-speechmatics-transcriber-failed")]
    PipelineErrorSpeechmaticsTranscriberFailed,
    #[serde(rename = "pipeline-error-assembly-ai-transcriber-failed")]
    PipelineErrorAssemblyAiTranscriberFailed,
    #[serde(rename = "pipeline-error-talkscriber-transcriber-failed")]
    PipelineErrorTalkscriberTranscriberFailed,
    #[serde(rename = "pipeline-error-azure-speech-transcriber-failed")]
    PipelineErrorAzureSpeechTranscriberFailed,
    #[serde(rename = "pipeline-error-vapi-llm-failed")]
    PipelineErrorVapiLlmFailed,
    #[serde(rename = "pipeline-error-vapi-400-bad-request-validation-failed")]
    PipelineErrorVapi400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-vapi-401-unauthorized")]
    PipelineErrorVapi401Unauthorized,
    #[serde(rename = "pipeline-error-vapi-403-model-access-denied")]
    PipelineErrorVapi403ModelAccessDenied,
    #[serde(rename = "pipeline-error-vapi-429-exceeded-quota")]
    PipelineErrorVapi429ExceededQuota,
    #[serde(rename = "pipeline-error-vapi-500-server-error")]
    PipelineErrorVapi500ServerError,
    #[serde(rename = "pipeline-no-available-model")]
    PipelineNoAvailableModel,
    #[serde(rename = "worker-shutdown")]
    WorkerShutdown,
    #[serde(rename = "unknown-error")]
    UnknownError,
    #[serde(rename = "vonage-disconnected")]
    VonageDisconnected,
    #[serde(rename = "vonage-failed-to-connect-call")]
    VonageFailedToConnectCall,
    #[serde(rename = "phone-call-provider-bypass-enabled-but-no-call-received")]
    PhoneCallProviderBypassEnabledButNoCallReceived,
    #[serde(rename = "vapifault-phone-call-worker-setup-socket-error")]
    VapifaultPhoneCallWorkerSetupSocketError,
    #[serde(rename = "vapifault-phone-call-worker-worker-setup-socket-timeout")]
    VapifaultPhoneCallWorkerWorkerSetupSocketTimeout,
    #[serde(rename = "vapifault-phone-call-worker-could-not-find-call")]
    VapifaultPhoneCallWorkerCouldNotFindCall,
    #[serde(rename = "vapifault-transport-never-connected")]
    VapifaultTransportNeverConnected,
    #[serde(rename = "vapifault-web-call-worker-setup-failed")]
    VapifaultWebCallWorkerSetupFailed,
    #[serde(rename = "vapifault-transport-connected-but-call-not-active")]
    VapifaultTransportConnectedButCallNotActive,
    #[serde(rename = "vapifault-call-started-but-connection-to-transport-missing")]
    VapifaultCallStartedButConnectionToTransportMissing,
    #[serde(rename = "pipeline-error-openai-llm-failed")]
    PipelineErrorOpenaiLlmFailed,
    #[serde(rename = "pipeline-error-azure-openai-llm-failed")]
    PipelineErrorAzureOpenaiLlmFailed,
    #[serde(rename = "pipeline-error-groq-llm-failed")]
    PipelineErrorGroqLlmFailed,
    #[serde(rename = "pipeline-error-google-llm-failed")]
    PipelineErrorGoogleLlmFailed,
    #[serde(rename = "pipeline-error-xai-llm-failed")]
    PipelineErrorXaiLlmFailed,
    #[serde(rename = "pipeline-error-mistral-llm-failed")]
    PipelineErrorMistralLlmFailed,
    #[serde(rename = "pipeline-error-inflection-ai-llm-failed")]
    PipelineErrorInflectionAiLlmFailed,
    #[serde(rename = "pipeline-error-cerebras-llm-failed")]
    PipelineErrorCerebrasLlmFailed,
    #[serde(rename = "pipeline-error-deep-seek-llm-failed")]
    PipelineErrorDeepSeekLlmFailed,
    #[serde(rename = "pipeline-error-openai-400-bad-request-validation-failed")]
    PipelineErrorOpenai400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-openai-401-unauthorized")]
    PipelineErrorOpenai401Unauthorized,
    #[serde(rename = "pipeline-error-openai-403-model-access-denied")]
    PipelineErrorOpenai403ModelAccessDenied,
    #[serde(rename = "pipeline-error-openai-429-exceeded-quota")]
    PipelineErrorOpenai429ExceededQuota,
    #[serde(rename = "pipeline-error-openai-500-server-error")]
    PipelineErrorOpenai500ServerError,
    #[serde(rename = "pipeline-error-google-400-bad-request-validation-failed")]
    PipelineErrorGoogle400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-google-401-unauthorized")]
    PipelineErrorGoogle401Unauthorized,
    #[serde(rename = "pipeline-error-google-403-model-access-denied")]
    PipelineErrorGoogle403ModelAccessDenied,
    #[serde(rename = "pipeline-error-google-429-exceeded-quota")]
    PipelineErrorGoogle429ExceededQuota,
    #[serde(rename = "pipeline-error-google-500-server-error")]
    PipelineErrorGoogle500ServerError,
    #[serde(rename = "pipeline-error-xai-400-bad-request-validation-failed")]
    PipelineErrorXai400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-xai-401-unauthorized")]
    PipelineErrorXai401Unauthorized,
    #[serde(rename = "pipeline-error-xai-403-model-access-denied")]
    PipelineErrorXai403ModelAccessDenied,
    #[serde(rename = "pipeline-error-xai-429-exceeded-quota")]
    PipelineErrorXai429ExceededQuota,
    #[serde(rename = "pipeline-error-xai-500-server-error")]
    PipelineErrorXai500ServerError,
    #[serde(rename = "pipeline-error-mistral-400-bad-request-validation-failed")]
    PipelineErrorMistral400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-mistral-401-unauthorized")]
    PipelineErrorMistral401Unauthorized,
    #[serde(rename = "pipeline-error-mistral-403-model-access-denied")]
    PipelineErrorMistral403ModelAccessDenied,
    #[serde(rename = "pipeline-error-mistral-429-exceeded-quota")]
    PipelineErrorMistral429ExceededQuota,
    #[serde(rename = "pipeline-error-mistral-500-server-error")]
    PipelineErrorMistral500ServerError,
    #[serde(rename = "pipeline-error-inflection-ai-400-bad-request-validation-failed")]
    PipelineErrorInflectionAi400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-inflection-ai-401-unauthorized")]
    PipelineErrorInflectionAi401Unauthorized,
    #[serde(rename = "pipeline-error-inflection-ai-403-model-access-denied")]
    PipelineErrorInflectionAi403ModelAccessDenied,
    #[serde(rename = "pipeline-error-inflection-ai-429-exceeded-quota")]
    PipelineErrorInflectionAi429ExceededQuota,
    #[serde(rename = "pipeline-error-inflection-ai-500-server-error")]
    PipelineErrorInflectionAi500ServerError,
    #[serde(rename = "pipeline-error-deep-seek-400-bad-request-validation-failed")]
    PipelineErrorDeepSeek400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-deep-seek-401-unauthorized")]
    PipelineErrorDeepSeek401Unauthorized,
    #[serde(rename = "pipeline-error-deep-seek-403-model-access-denied")]
    PipelineErrorDeepSeek403ModelAccessDenied,
    #[serde(rename = "pipeline-error-deep-seek-429-exceeded-quota")]
    PipelineErrorDeepSeek429ExceededQuota,
    #[serde(rename = "pipeline-error-deep-seek-500-server-error")]
    PipelineErrorDeepSeek500ServerError,
    #[serde(rename = "pipeline-error-azure-openai-400-bad-request-validation-failed")]
    PipelineErrorAzureOpenai400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-azure-openai-401-unauthorized")]
    PipelineErrorAzureOpenai401Unauthorized,
    #[serde(rename = "pipeline-error-azure-openai-403-model-access-denied")]
    PipelineErrorAzureOpenai403ModelAccessDenied,
    #[serde(rename = "pipeline-error-azure-openai-429-exceeded-quota")]
    PipelineErrorAzureOpenai429ExceededQuota,
    #[serde(rename = "pipeline-error-azure-openai-500-server-error")]
    PipelineErrorAzureOpenai500ServerError,
    #[serde(rename = "pipeline-error-groq-400-bad-request-validation-failed")]
    PipelineErrorGroq400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-groq-401-unauthorized")]
    PipelineErrorGroq401Unauthorized,
    #[serde(rename = "pipeline-error-groq-403-model-access-denied")]
    PipelineErrorGroq403ModelAccessDenied,
    #[serde(rename = "pipeline-error-groq-429-exceeded-quota")]
    PipelineErrorGroq429ExceededQuota,
    #[serde(rename = "pipeline-error-groq-500-server-error")]
    PipelineErrorGroq500ServerError,
    #[serde(rename = "pipeline-error-cerebras-400-bad-request-validation-failed")]
    PipelineErrorCerebras400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-cerebras-401-unauthorized")]
    PipelineErrorCerebras401Unauthorized,
    #[serde(rename = "pipeline-error-cerebras-403-model-access-denied")]
    PipelineErrorCerebras403ModelAccessDenied,
    #[serde(rename = "pipeline-error-cerebras-429-exceeded-quota")]
    PipelineErrorCerebras429ExceededQuota,
    #[serde(rename = "pipeline-error-cerebras-500-server-error")]
    PipelineErrorCerebras500ServerError,
    #[serde(rename = "pipeline-error-anthropic-400-bad-request-validation-failed")]
    PipelineErrorAnthropic400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-anthropic-401-unauthorized")]
    PipelineErrorAnthropic401Unauthorized,
    #[serde(rename = "pipeline-error-anthropic-403-model-access-denied")]
    PipelineErrorAnthropic403ModelAccessDenied,
    #[serde(rename = "pipeline-error-anthropic-429-exceeded-quota")]
    PipelineErrorAnthropic429ExceededQuota,
    #[serde(rename = "pipeline-error-anthropic-500-server-error")]
    PipelineErrorAnthropic500ServerError,
    #[serde(rename = "pipeline-error-anthropic-llm-failed")]
    PipelineErrorAnthropicLlmFailed,
    #[serde(rename = "pipeline-error-together-ai-400-bad-request-validation-failed")]
    PipelineErrorTogetherAi400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-together-ai-401-unauthorized")]
    PipelineErrorTogetherAi401Unauthorized,
    #[serde(rename = "pipeline-error-together-ai-403-model-access-denied")]
    PipelineErrorTogetherAi403ModelAccessDenied,
    #[serde(rename = "pipeline-error-together-ai-429-exceeded-quota")]
    PipelineErrorTogetherAi429ExceededQuota,
    #[serde(rename = "pipeline-error-together-ai-500-server-error")]
    PipelineErrorTogetherAi500ServerError,
    #[serde(rename = "pipeline-error-together-ai-llm-failed")]
    PipelineErrorTogetherAiLlmFailed,
    #[serde(rename = "pipeline-error-anyscale-400-bad-request-validation-failed")]
    PipelineErrorAnyscale400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-anyscale-401-unauthorized")]
    PipelineErrorAnyscale401Unauthorized,
    #[serde(rename = "pipeline-error-anyscale-403-model-access-denied")]
    PipelineErrorAnyscale403ModelAccessDenied,
    #[serde(rename = "pipeline-error-anyscale-429-exceeded-quota")]
    PipelineErrorAnyscale429ExceededQuota,
    #[serde(rename = "pipeline-error-anyscale-500-server-error")]
    PipelineErrorAnyscale500ServerError,
    #[serde(rename = "pipeline-error-anyscale-llm-failed")]
    PipelineErrorAnyscaleLlmFailed,
    #[serde(rename = "pipeline-error-openrouter-400-bad-request-validation-failed")]
    PipelineErrorOpenrouter400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-openrouter-401-unauthorized")]
    PipelineErrorOpenrouter401Unauthorized,
    #[serde(rename = "pipeline-error-openrouter-403-model-access-denied")]
    PipelineErrorOpenrouter403ModelAccessDenied,
    #[serde(rename = "pipeline-error-openrouter-429-exceeded-quota")]
    PipelineErrorOpenrouter429ExceededQuota,
    #[serde(rename = "pipeline-error-openrouter-500-server-error")]
    PipelineErrorOpenrouter500ServerError,
    #[serde(rename = "pipeline-error-openrouter-llm-failed")]
    PipelineErrorOpenrouterLlmFailed,
    #[serde(rename = "pipeline-error-perplexity-ai-400-bad-request-validation-failed")]
    PipelineErrorPerplexityAi400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-perplexity-ai-401-unauthorized")]
    PipelineErrorPerplexityAi401Unauthorized,
    #[serde(rename = "pipeline-error-perplexity-ai-403-model-access-denied")]
    PipelineErrorPerplexityAi403ModelAccessDenied,
    #[serde(rename = "pipeline-error-perplexity-ai-429-exceeded-quota")]
    PipelineErrorPerplexityAi429ExceededQuota,
    #[serde(rename = "pipeline-error-perplexity-ai-500-server-error")]
    PipelineErrorPerplexityAi500ServerError,
    #[serde(rename = "pipeline-error-perplexity-ai-llm-failed")]
    PipelineErrorPerplexityAiLlmFailed,
    #[serde(rename = "pipeline-error-deepinfra-400-bad-request-validation-failed")]
    PipelineErrorDeepinfra400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-deepinfra-401-unauthorized")]
    PipelineErrorDeepinfra401Unauthorized,
    #[serde(rename = "pipeline-error-deepinfra-403-model-access-denied")]
    PipelineErrorDeepinfra403ModelAccessDenied,
    #[serde(rename = "pipeline-error-deepinfra-429-exceeded-quota")]
    PipelineErrorDeepinfra429ExceededQuota,
    #[serde(rename = "pipeline-error-deepinfra-500-server-error")]
    PipelineErrorDeepinfra500ServerError,
    #[serde(rename = "pipeline-error-deepinfra-llm-failed")]
    PipelineErrorDeepinfraLlmFailed,
    #[serde(rename = "pipeline-error-runpod-400-bad-request-validation-failed")]
    PipelineErrorRunpod400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-runpod-401-unauthorized")]
    PipelineErrorRunpod401Unauthorized,
    #[serde(rename = "pipeline-error-runpod-403-model-access-denied")]
    PipelineErrorRunpod403ModelAccessDenied,
    #[serde(rename = "pipeline-error-runpod-429-exceeded-quota")]
    PipelineErrorRunpod429ExceededQuota,
    #[serde(rename = "pipeline-error-runpod-500-server-error")]
    PipelineErrorRunpod500ServerError,
    #[serde(rename = "pipeline-error-runpod-llm-failed")]
    PipelineErrorRunpodLlmFailed,
    #[serde(rename = "pipeline-error-custom-llm-400-bad-request-validation-failed")]
    PipelineErrorCustomLlm400BadRequestValidationFailed,
    #[serde(rename = "pipeline-error-custom-llm-401-unauthorized")]
    PipelineErrorCustomLlm401Unauthorized,
    #[serde(rename = "pipeline-error-custom-llm-403-model-access-denied")]
    PipelineErrorCustomLlm403ModelAccessDenied,
    #[serde(rename = "pipeline-error-custom-llm-429-exceeded-quota")]
    PipelineErrorCustomLlm429ExceededQuota,
    #[serde(rename = "pipeline-error-custom-llm-500-server-error")]
    PipelineErrorCustomLlm500ServerError,
    #[serde(rename = "pipeline-error-custom-llm-llm-failed")]
    PipelineErrorCustomLlmLlmFailed,
    #[serde(rename = "pipeline-error-custom-voice-failed")]
    PipelineErrorCustomVoiceFailed,
    #[serde(rename = "pipeline-error-cartesia-socket-hang-up")]
    PipelineErrorCartesiaSocketHangUp,
    #[serde(rename = "pipeline-error-cartesia-requested-payment")]
    PipelineErrorCartesiaRequestedPayment,
    #[serde(rename = "pipeline-error-cartesia-500-server-error")]
    PipelineErrorCartesia500ServerError,
    #[serde(rename = "pipeline-error-cartesia-503-server-error")]
    PipelineErrorCartesia503ServerError,
    #[serde(rename = "pipeline-error-cartesia-522-server-error")]
    PipelineErrorCartesia522ServerError,
    #[serde(rename = "pipeline-error-eleven-labs-voice-not-found")]
    PipelineErrorElevenLabsVoiceNotFound,
    #[serde(rename = "pipeline-error-eleven-labs-quota-exceeded")]
    PipelineErrorElevenLabsQuotaExceeded,
    #[serde(rename = "pipeline-error-eleven-labs-unauthorized-access")]
    PipelineErrorElevenLabsUnauthorizedAccess,
    #[serde(rename = "pipeline-error-eleven-labs-unauthorized-to-access-model")]
    PipelineErrorElevenLabsUnauthorizedToAccessModel,
    #[serde(rename = "pipeline-error-eleven-labs-professional-voices-only-for-creator-plus")]
    PipelineErrorElevenLabsProfessionalVoicesOnlyForCreatorPlus,
    #[serde(rename = "pipeline-error-eleven-labs-blocked-free-plan-and-requested-upgrade")]
    PipelineErrorElevenLabsBlockedFreePlanAndRequestedUpgrade,
    #[serde(
        rename = "pipeline-error-eleven-labs-blocked-concurrent-requests-and-requested-upgrade"
    )]
    PipelineErrorElevenLabsBlockedConcurrentRequestsAndRequestedUpgrade,
    #[serde(
        rename = "pipeline-error-eleven-labs-blocked-using-instant-voice-clone-and-requested-upgrade"
    )]
    PipelineErrorElevenLabsBlockedUsingInstantVoiceCloneAndRequestedUpgrade,
    #[serde(rename = "pipeline-error-eleven-labs-system-busy-and-requested-upgrade")]
    PipelineErrorElevenLabsSystemBusyAndRequestedUpgrade,
    #[serde(rename = "pipeline-error-eleven-labs-voice-not-fine-tuned")]
    PipelineErrorElevenLabsVoiceNotFineTuned,
    #[serde(rename = "pipeline-error-eleven-labs-invalid-api-key")]
    PipelineErrorElevenLabsInvalidApiKey,
    #[serde(rename = "pipeline-error-eleven-labs-invalid-voice-samples")]
    PipelineErrorElevenLabsInvalidVoiceSamples,
    #[serde(rename = "pipeline-error-eleven-labs-voice-disabled-by-owner")]
    PipelineErrorElevenLabsVoiceDisabledByOwner,
    #[serde(rename = "pipeline-error-eleven-labs-blocked-account-in-probation")]
    PipelineErrorElevenLabsBlockedAccountInProbation,
    #[serde(rename = "pipeline-error-eleven-labs-blocked-content-against-their-policy")]
    PipelineErrorElevenLabsBlockedContentAgainstTheirPolicy,
    #[serde(rename = "pipeline-error-eleven-labs-missing-samples-for-voice-clone")]
    PipelineErrorElevenLabsMissingSamplesForVoiceClone,
    #[serde(rename = "pipeline-error-eleven-labs-voice-not-fine-tuned-and-cannot-be-used")]
    PipelineErrorElevenLabsVoiceNotFineTunedAndCannotBeUsed,
    #[serde(rename = "pipeline-error-eleven-labs-voice-not-allowed-for-free-users")]
    PipelineErrorElevenLabsVoiceNotAllowedForFreeUsers,
    #[serde(rename = "pipeline-error-eleven-labs-500-server-error")]
    PipelineErrorElevenLabs500ServerError,
    #[serde(rename = "pipeline-error-eleven-labs-max-character-limit-exceeded")]
    PipelineErrorElevenLabsMaxCharacterLimitExceeded,
    #[serde(
        rename = "pipeline-error-eleven-labs-blocked-voice-potentially-against-terms-of-service-and-awaiting-verification"
    )]
    PipelineErrorElevenLabsBlockedVoicePotentiallyAgainstTermsOfServiceAndAwaitingVerification,
    #[serde(rename = "pipeline-error-playht-request-timed-out")]
    PipelineErrorPlayhtRequestTimedOut,
    #[serde(rename = "pipeline-error-playht-invalid-voice")]
    PipelineErrorPlayhtInvalidVoice,
    #[serde(rename = "pipeline-error-playht-unexpected-error")]
    PipelineErrorPlayhtUnexpectedError,
    #[serde(rename = "pipeline-error-playht-out-of-credits")]
    PipelineErrorPlayhtOutOfCredits,
    #[serde(rename = "pipeline-error-playht-invalid-emotion")]
    PipelineErrorPlayhtInvalidEmotion,
    #[serde(rename = "pipeline-error-playht-voice-must-be-a-valid-voice-manifest-uri")]
    PipelineErrorPlayhtVoiceMustBeAValidVoiceManifestUri,
    #[serde(rename = "pipeline-error-playht-401-unauthorized")]
    PipelineErrorPlayht401Unauthorized,
    #[serde(rename = "pipeline-error-playht-403-forbidden-out-of-characters")]
    PipelineErrorPlayht403ForbiddenOutOfCharacters,
    #[serde(rename = "pipeline-error-playht-403-forbidden-api-access-not-available")]
    PipelineErrorPlayht403ForbiddenApiAccessNotAvailable,
    #[serde(rename = "pipeline-error-playht-429-exceeded-quota")]
    PipelineErrorPlayht429ExceededQuota,
    #[serde(rename = "pipeline-error-playht-502-gateway-error")]
    PipelineErrorPlayht502GatewayError,
    #[serde(rename = "pipeline-error-playht-504-gateway-error")]
    PipelineErrorPlayht504GatewayError,
    #[serde(rename = "pipeline-error-tavus-video-failed")]
    PipelineErrorTavusVideoFailed,
    #[serde(rename = "pipeline-error-custom-transcriber-failed")]
    PipelineErrorCustomTranscriberFailed,
    #[serde(rename = "pipeline-error-11labs-transcriber-failed")]
    PipelineError11labsTranscriberFailed,
    #[serde(rename = "pipeline-error-deepgram-returning-403-model-access-denied")]
    PipelineErrorDeepgramReturning403ModelAccessDenied,
    #[serde(rename = "pipeline-error-deepgram-returning-401-invalid-credentials")]
    PipelineErrorDeepgramReturning401InvalidCredentials,
    #[serde(rename = "pipeline-error-deepgram-returning-404-not-found")]
    PipelineErrorDeepgramReturning404NotFound,
    #[serde(
        rename = "pipeline-error-deepgram-returning-400-no-such-model-language-tier-combination"
    )]
    PipelineErrorDeepgramReturning400NoSuchModelLanguageTierCombination,
    #[serde(rename = "pipeline-error-deepgram-returning-500-invalid-json")]
    PipelineErrorDeepgramReturning500InvalidJson,
    #[serde(rename = "pipeline-error-deepgram-returning-502-network-error")]
    PipelineErrorDeepgramReturning502NetworkError,
    #[serde(rename = "pipeline-error-deepgram-returning-502-bad-gateway-ehostunreach")]
    PipelineErrorDeepgramReturning502BadGatewayEhostunreach,
    #[serde(rename = "silence-timed-out")]
    SilenceTimedOut,
    #[serde(rename = "sip-gateway-failed-to-connect-call")]
    SipGatewayFailedToConnectCall,
    #[serde(rename = "twilio-failed-to-connect-call")]
    TwilioFailedToConnectCall,
    #[serde(rename = "twilio-reported-customer-misdialed")]
    TwilioReportedCustomerMisdialed,
    #[serde(rename = "vonage-rejected")]
    VonageRejected,
    #[serde(rename = "voicemail")]
    Voicemail,
}

impl Default for EndedReason {
    fn default() -> EndedReason {
        Self::AssistantNotValid
    }
}
