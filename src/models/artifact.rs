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
pub struct Artifact {
    /// These are the messages that were spoken during the call.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::ArtifactMessagesInner>>,
    /// These are the messages that were spoken during the call, formatted for OpenAI.
    #[serde(
        rename = "messagesOpenAIFormatted",
        skip_serializing_if = "Option::is_none"
    )]
    pub messages_open_ai_formatted: Option<Vec<models::OpenAiMessage>>,
    /// This is the recording url for the call. To enable, set `assistant.artifactPlan.recordingEnabled`.
    #[serde(rename = "recordingUrl", skip_serializing_if = "Option::is_none")]
    pub recording_url: Option<String>,
    /// This is the stereo recording url for the call. To enable, set `assistant.artifactPlan.recordingEnabled`.
    #[serde(rename = "stereoRecordingUrl", skip_serializing_if = "Option::is_none")]
    pub stereo_recording_url: Option<String>,
    /// This is video recording url for the call. To enable, set `assistant.artifactPlan.videoRecordingEnabled`.
    #[serde(rename = "videoRecordingUrl", skip_serializing_if = "Option::is_none")]
    pub video_recording_url: Option<String>,
    /// This is video recording start delay in ms. To enable, set `assistant.artifactPlan.videoRecordingEnabled`. This can be used to align the playback of the recording with artifact.messages timestamps.
    #[serde(
        rename = "videoRecordingStartDelaySeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_recording_start_delay_seconds: Option<f64>,
    /// This is the transcript of the call. This is derived from `artifact.messages` but provided for convenience.
    #[serde(rename = "transcript", skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// This is the packet capture url for the call. This is only available for `phone` type calls where phone number's provider is `vapi` or `byo-phone-number`.
    #[serde(rename = "pcapUrl", skip_serializing_if = "Option::is_none")]
    pub pcap_url: Option<String>,
}

impl Artifact {
    pub fn new() -> Artifact {
        Artifact {
            messages: None,
            messages_open_ai_formatted: None,
            recording_url: None,
            stereo_recording_url: None,
            video_recording_url: None,
            video_recording_start_delay_seconds: None,
            transcript: None,
            pcap_url: None,
        }
    }
}
