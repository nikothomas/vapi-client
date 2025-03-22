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
pub struct TransportConfigurationTwilio {
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// The integer number of seconds that we should allow the phone to ring before assuming there is no answer. The default is `60` seconds and the maximum is `600` seconds. For some call flows, we will add a 5-second buffer to the timeout value you provide. For this reason, a timeout value of 10 seconds could result in an actual timeout closer to 15 seconds. You can set this to a short time, such as `15` seconds, to hang up before reaching an answering machine or voicemail.  @default 60
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    /// Whether to record the call. Can be `true` to record the phone call, or `false` to not. The default is `false`.  @default false
    #[serde(rename = "record", skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    /// The number of channels in the final recording. Can be: `mono` or `dual`. The default is `mono`. `mono` records both legs of the call in a single channel of the recording file. `dual` records each leg to a separate channel of the recording file. The first channel of a dual-channel recording contains the parent call and the second channel contains the child call.  @default 'mono'
    #[serde(rename = "recordingChannels", skip_serializing_if = "Option::is_none")]
    pub recording_channels: Option<RecordingChannels>,
}

impl TransportConfigurationTwilio {
    pub fn new(provider: Provider) -> TransportConfigurationTwilio {
        TransportConfigurationTwilio {
            provider,
            timeout: None,
            record: None,
            recording_channels: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "twilio")]
    Twilio,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Twilio
    }
}
/// The number of channels in the final recording. Can be: `mono` or `dual`. The default is `mono`. `mono` records both legs of the call in a single channel of the recording file. `dual` records each leg to a separate channel of the recording file. The first channel of a dual-channel recording contains the parent call and the second channel contains the child call.  @default 'mono'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum RecordingChannels {
    #[serde(rename = "mono")]
    Mono,
    #[serde(rename = "dual")]
    Dual,
}

impl Default for RecordingChannels {
    fn default() -> RecordingChannels {
        Self::Mono
    }
}
