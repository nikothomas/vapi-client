/*
 * Vapi API
 *
 * Voice AI for developers.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantDtoTransportConfigurationsInner {
    TransportConfigurationTwilio(models::TransportConfigurationTwilio),
}

impl Default for CreateAssistantDtoTransportConfigurationsInner {
    fn default() -> Self {
        Self::TransportConfigurationTwilio(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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

