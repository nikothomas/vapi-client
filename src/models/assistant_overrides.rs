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
use utoipa::ToSchema;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct AssistantOverrides {
    #[serde(rename = "transcriber", skip_serializing_if = "Option::is_none")]
    pub transcriber: Option<models::CreateAssistantDtoTranscriber>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::CreateAssistantDtoModel>,
    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<models::CreateAssistantDtoVoice>,
    /// This is the first message that the assistant will say. This can also be a URL to a containerized audio file (mp3, wav, etc.).  If unspecified, assistant will wait for user to speak and use the model to respond once they speak.
    #[serde(rename = "firstMessage", skip_serializing_if = "Option::is_none")]
    pub first_message: Option<String>,
    /// This is the mode for the first message. Default is 'assistant-speaks-first'.  Use: - 'assistant-speaks-first' to have the assistant speak first. - 'assistant-waits-for-user' to have the assistant wait for the user to speak first. - 'assistant-speaks-first-with-model-generated-message' to have the assistant speak first with a message generated by the model based on the conversation state. (`assistant.model.messages` at call start, `call.messages` at squad transfer points).  @default 'assistant-speaks-first'
    #[serde(rename = "firstMessageMode", skip_serializing_if = "Option::is_none")]
    pub first_message_mode: Option<FirstMessageMode>,
    /// These are the settings to configure or disable voicemail detection. Alternatively, voicemail detection can be configured using the model.tools=[VoicemailTool]. This uses Twilio's built-in detection while the VoicemailTool relies on the model to detect if a voicemail was reached. You can use neither of them, one of them, or both of them. By default, Twilio built-in detection is enabled while VoicemailTool is not.
    #[serde(rename = "voicemailDetection", skip_serializing_if = "Option::is_none")]
    pub voicemail_detection: Option<serde_json::Value>,
    /// These are the messages that will be sent to your Client SDKs. Default is conversation-update,function-call,hang,model-output,speech-update,status-update,transfer-update,transcript,tool-calls,user-interrupted,voice-input. You can check the shape of the messages in ClientMessage schema.
    #[serde(rename = "clientMessages", skip_serializing_if = "Option::is_none")]
    pub client_messages: Option<Vec<ClientMessages>>,
    /// These are the messages that will be sent to your Server URL. Default is conversation-update,end-of-call-report,function-call,hang,speech-update,status-update,tool-calls,transfer-destination-request,user-interrupted. You can check the shape of the messages in ServerMessage schema.
    #[serde(rename = "serverMessages", skip_serializing_if = "Option::is_none")]
    pub server_messages: Option<Vec<ServerMessages>>,
    /// How many seconds of silence to wait before ending the call. Defaults to 30.  @default 30
    #[serde(
        rename = "silenceTimeoutSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub silence_timeout_seconds: Option<f64>,
    /// This is the maximum number of seconds that the call will last. When the call reaches this duration, it will be ended.  @default 600 (10 minutes)
    #[serde(rename = "maxDurationSeconds", skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<f64>,
    /// This is the background sound in the call. Default for phone calls is 'office' and default for web calls is 'off'.
    #[serde(rename = "backgroundSound", skip_serializing_if = "Option::is_none")]
    pub background_sound: Option<BackgroundSound>,
    /// This enables filtering of noise and background speech while the user is talking.  Default `false` while in beta.  @default false
    #[serde(
        rename = "backgroundDenoisingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_denoising_enabled: Option<bool>,
    /// This determines whether the model's output is used in conversation history rather than the transcription of assistant's speech.  Default `false` while in beta.  @default false
    #[serde(
        rename = "modelOutputInMessagesEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_output_in_messages_enabled: Option<bool>,
    /// These are the configurations to be passed to the transport providers of assistant's calls, like Twilio. You can store multiple configurations for different transport providers. For a call, only the configuration matching the call transport provider is used.
    #[serde(
        rename = "transportConfigurations",
        skip_serializing_if = "Option::is_none"
    )]
    pub transport_configurations:
        Option<Vec<models::CreateAssistantDtoTransportConfigurationsInner>>,
    /// These are dynamic credentials that will be used for the assistant calls. By default, all the credentials are available for use in the call but you can supplement an additional credentials using this. Dynamic credentials override existing credentials.
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<models::CreateAssistantDtoCredentialsInner>>,
    /// These are values that will be used to replace the template variables in the assistant messages and other text-based fields. This uses LiquidJS syntax. https://liquidjs.com/tutorials/intro-to-liquid.html  So for example, `{{ name }}` will be replaced with the value of `name` in `variableValues`. `{{\"now\" | date: \"%b %d, %Y, %I:%M %p\", \"America/New_York\"}}` will be replaced with the current date and time in New York.  Some Vapi reserved defaults:  - *customer* - the customer object
    #[serde(rename = "variableValues", skip_serializing_if = "Option::is_none")]
    pub variable_values: Option<serde_json::Value>,
    /// This is the name of the assistant.  This is required when you want to transfer between assistants in a call.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This is the message that the assistant will say if the call is forwarded to voicemail.  If unspecified, it will hang up.
    #[serde(rename = "voicemailMessage", skip_serializing_if = "Option::is_none")]
    pub voicemail_message: Option<String>,
    /// This is the message that the assistant will say if it ends the call.  If unspecified, it will hang up without saying anything.
    #[serde(rename = "endCallMessage", skip_serializing_if = "Option::is_none")]
    pub end_call_message: Option<String>,
    /// This list contains phrases that, if spoken by the assistant, will trigger the call to be hung up. Case insensitive.
    #[serde(rename = "endCallPhrases", skip_serializing_if = "Option::is_none")]
    pub end_call_phrases: Option<Vec<String>>,
    #[serde(rename = "compliancePlan", skip_serializing_if = "Option::is_none")]
    pub compliance_plan: Option<models::CompliancePlan>,
    /// This is for metadata you want to store on the assistant.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// This is the plan for analysis of assistant's calls. Stored in `call.analysis`.
    #[serde(rename = "analysisPlan", skip_serializing_if = "Option::is_none")]
    pub analysis_plan: Option<models::AnalysisPlan>,
    /// This is the plan for artifacts generated during assistant's calls. Stored in `call.artifact`.  Note: `recordingEnabled` is currently at the root level. It will be moved to `artifactPlan` in the future, but will remain backwards compatible.
    #[serde(rename = "artifactPlan", skip_serializing_if = "Option::is_none")]
    pub artifact_plan: Option<models::ArtifactPlan>,
    /// This is the plan for static predefined messages that can be spoken by the assistant during the call, like `idleMessages`.  Note: `firstMessage`, `voicemailMessage`, and `endCallMessage` are currently at the root level. They will be moved to `messagePlan` in the future, but will remain backwards compatible.
    #[serde(rename = "messagePlan", skip_serializing_if = "Option::is_none")]
    pub message_plan: Option<models::MessagePlan>,
    /// This is the plan for when the assistant should start talking.  You should configure this if you're running into these issues: - The assistant is too slow to start talking after the customer is done speaking. - The assistant is too fast to start talking after the customer is done speaking. - The assistant is so fast that it's actually interrupting the customer.
    #[serde(rename = "startSpeakingPlan", skip_serializing_if = "Option::is_none")]
    pub start_speaking_plan: Option<models::StartSpeakingPlan>,
    /// This is the plan for when assistant should stop talking on customer interruption.  You should configure this if you're running into these issues: - The assistant is too slow to recognize customer's interruption. - The assistant is too fast to recognize customer's interruption. - The assistant is getting interrupted by phrases that are just acknowledgments. - The assistant is getting interrupted by background noises. - The assistant is not properly stopping -- it starts talking right after getting interrupted.
    #[serde(rename = "stopSpeakingPlan", skip_serializing_if = "Option::is_none")]
    pub stop_speaking_plan: Option<models::StopSpeakingPlan>,
    /// This is the plan for real-time monitoring of the assistant's calls.  Usage: - To enable live listening of the assistant's calls, set `monitorPlan.listenEnabled` to `true`. - To enable live control of the assistant's calls, set `monitorPlan.controlEnabled` to `true`.  Note, `serverMessages`, `clientMessages`, `serverUrl` and `serverUrlSecret` are currently at the root level but will be moved to `monitorPlan` in the future. Will remain backwards compatible
    #[serde(rename = "monitorPlan", skip_serializing_if = "Option::is_none")]
    pub monitor_plan: Option<models::MonitorPlan>,
    /// These are the credentials that will be used for the assistant calls. By default, all the credentials are available for use in the call but you can provide a subset using this.
    #[serde(rename = "credentialIds", skip_serializing_if = "Option::is_none")]
    pub credential_ids: Option<Vec<String>>,
    /// This is where Vapi will send webhooks. You can find all webhooks available along with their shape in ServerMessage schema.  The order of precedence is:  1. assistant.server.url 2. phoneNumber.serverUrl 3. org.serverUrl
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
    /// This is a set of actions that will be performed on certain events.
    #[serde(rename = "hooks", skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Vec<models::AssistantHooks>>,
    #[serde(rename = "keypadInputPlan", skip_serializing_if = "Option::is_none")]
    pub keypad_input_plan: Option<models::KeypadInputPlan>,
}

impl AssistantOverrides {
    pub fn new() -> AssistantOverrides {
        AssistantOverrides {
            transcriber: None,
            model: None,
            voice: None,
            first_message: None,
            first_message_mode: None,
            voicemail_detection: None,
            client_messages: None,
            server_messages: None,
            silence_timeout_seconds: None,
            max_duration_seconds: None,
            background_sound: None,
            background_denoising_enabled: None,
            model_output_in_messages_enabled: None,
            transport_configurations: None,
            credentials: None,
            variable_values: None,
            name: None,
            voicemail_message: None,
            end_call_message: None,
            end_call_phrases: None,
            compliance_plan: None,
            metadata: None,
            analysis_plan: None,
            artifact_plan: None,
            message_plan: None,
            start_speaking_plan: None,
            stop_speaking_plan: None,
            monitor_plan: None,
            credential_ids: None,
            server: None,
            hooks: None,
            keypad_input_plan: None,
        }
    }
}
/// This is the mode for the first message. Default is 'assistant-speaks-first'.  Use: - 'assistant-speaks-first' to have the assistant speak first. - 'assistant-waits-for-user' to have the assistant wait for the user to speak first. - 'assistant-speaks-first-with-model-generated-message' to have the assistant speak first with a message generated by the model based on the conversation state. (`assistant.model.messages` at call start, `call.messages` at squad transfer points).  @default 'assistant-speaks-first'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum FirstMessageMode {
    #[serde(rename = "assistant-speaks-first")]
    AssistantSpeaksFirst,
    #[serde(rename = "assistant-speaks-first-with-model-generated-message")]
    AssistantSpeaksFirstWithModelGeneratedMessage,
    #[serde(rename = "assistant-waits-for-user")]
    AssistantWaitsForUser,
}

impl Default for FirstMessageMode {
    fn default() -> FirstMessageMode {
        Self::AssistantSpeaksFirst
    }
}
/// These are the messages that will be sent to your Client SDKs. Default is conversation-update,function-call,hang,model-output,speech-update,status-update,transfer-update,transcript,tool-calls,user-interrupted,voice-input. You can check the shape of the messages in ClientMessage schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum ClientMessages {
    #[serde(rename = "conversation-update")]
    ConversationUpdate,
    #[serde(rename = "function-call")]
    FunctionCall,
    #[serde(rename = "function-call-result")]
    FunctionCallResult,
    #[serde(rename = "hang")]
    Hang,
    #[serde(rename = "language-changed")]
    LanguageChanged,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "model-output")]
    ModelOutput,
    #[serde(rename = "speech-update")]
    SpeechUpdate,
    #[serde(rename = "status-update")]
    StatusUpdate,
    #[serde(rename = "transcript")]
    Transcript,
    #[serde(rename = "tool-calls")]
    ToolCalls,
    #[serde(rename = "tool-calls-result")]
    ToolCallsResult,
    #[serde(rename = "transfer-update")]
    TransferUpdate,
    #[serde(rename = "user-interrupted")]
    UserInterrupted,
    #[serde(rename = "voice-input")]
    VoiceInput,
}

impl Default for ClientMessages {
    fn default() -> ClientMessages {
        Self::ConversationUpdate
    }
}
/// These are the messages that will be sent to your Server URL. Default is conversation-update,end-of-call-report,function-call,hang,speech-update,status-update,tool-calls,transfer-destination-request,user-interrupted. You can check the shape of the messages in ServerMessage schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum ServerMessages {
    #[serde(rename = "conversation-update")]
    ConversationUpdate,
    #[serde(rename = "end-of-call-report")]
    EndOfCallReport,
    #[serde(rename = "function-call")]
    FunctionCall,
    #[serde(rename = "hang")]
    Hang,
    #[serde(rename = "language-changed")]
    LanguageChanged,
    #[serde(rename = "language-change-detected")]
    LanguageChangeDetected,
    #[serde(rename = "model-output")]
    ModelOutput,
    #[serde(rename = "phone-call-control")]
    PhoneCallControl,
    #[serde(rename = "speech-update")]
    SpeechUpdate,
    #[serde(rename = "status-update")]
    StatusUpdate,
    #[serde(rename = "transcript")]
    Transcript,
    #[serde(rename = "transcript[transcriptType=\"final\"]")]
    TranscriptLeftSquareBracketTranscriptTypeEqualDoubleQuoteFinalDoubleQuoteRightSquareBracket,
    #[serde(rename = "tool-calls")]
    ToolCalls,
    #[serde(rename = "transfer-destination-request")]
    TransferDestinationRequest,
    #[serde(rename = "transfer-update")]
    TransferUpdate,
    #[serde(rename = "user-interrupted")]
    UserInterrupted,
    #[serde(rename = "voice-input")]
    VoiceInput,
}

impl Default for ServerMessages {
    fn default() -> ServerMessages {
        Self::ConversationUpdate
    }
}
/// This is the background sound in the call. Default for phone calls is 'office' and default for web calls is 'off'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema)]
pub enum BackgroundSound {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "office")]
    Office,
}

impl Default for BackgroundSound {
    fn default() -> BackgroundSound {
        Self::Off
    }
}
