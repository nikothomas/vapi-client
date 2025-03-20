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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TavusConversationProperties {
    /// The maximum duration of the call in seconds. The default `maxCallDuration` is 3600 seconds (1 hour). Once the time limit specified by this parameter has been reached, the conversation will automatically shut down.
    #[serde(rename = "maxCallDuration", skip_serializing_if = "Option::is_none")]
    pub max_call_duration: Option<f64>,
    /// The duration in seconds after which the call will be automatically shut down once the last participant leaves.
    #[serde(
        rename = "participantLeftTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub participant_left_timeout: Option<f64>,
    /// Starting from conversation creation, the duration in seconds after which the call will be automatically shut down if no participant joins the call. Default is 300 seconds (5 minutes).
    #[serde(
        rename = "participantAbsentTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub participant_absent_timeout: Option<f64>,
    /// If true, the user will be able to record the conversation.
    #[serde(rename = "enableRecording", skip_serializing_if = "Option::is_none")]
    pub enable_recording: Option<bool>,
    /// If true, the user will be able to transcribe the conversation. You can find more instructions on displaying transcriptions if you are using your custom DailyJS components here. You need to have an event listener on Daily that listens for `app-messages`.
    #[serde(
        rename = "enableTranscription",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_transcription: Option<bool>,
    /// If true, the background will be replaced with a greenscreen (RGB values: `[0, 255, 155]`). You can use WebGL on the frontend to make the greenscreen transparent or change its color.
    #[serde(rename = "applyGreenscreen", skip_serializing_if = "Option::is_none")]
    pub apply_greenscreen: Option<bool>,
    /// The language of the conversation. Please provide the **full language name**, not the two-letter code. If you are using your own TTS voice, please ensure it supports the language you provide. If you are using a stock replica or default persona, please note that only ElevenLabs and Cartesia supported languages are available. You can find a full list of supported languages for Cartesia here, for ElevenLabs here, and for PlayHT here.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The name of the S3 bucket where the recording will be stored.
    #[serde(
        rename = "recordingS3BucketName",
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_s3_bucket_name: Option<String>,
    /// The region of the S3 bucket where the recording will be stored.
    #[serde(
        rename = "recordingS3BucketRegion",
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_s3_bucket_region: Option<String>,
    /// The ARN of the role that will be assumed to access the S3 bucket.
    #[serde(rename = "awsAssumeRoleArn", skip_serializing_if = "Option::is_none")]
    pub aws_assume_role_arn: Option<String>,
}

impl TavusConversationProperties {
    pub fn new() -> TavusConversationProperties {
        TavusConversationProperties {
            max_call_duration: None,
            participant_left_timeout: None,
            participant_absent_timeout: None,
            enable_recording: None,
            enable_transcription: None,
            apply_greenscreen: None,
            language: None,
            recording_s3_bucket_name: None,
            recording_s3_bucket_region: None,
            aws_assume_role_arn: None,
        }
    }
}
