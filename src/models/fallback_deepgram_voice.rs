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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FallbackDeepgramVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the provider-specific ID that will be used.
    #[serde(rename = "voiceId")]
    pub voice_id: VoiceId,
    /// This is the model that will be used. Defaults to 'aura-2' when not specified.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    /// If set to true, this will add mip_opt_out=true as a query parameter of all API requests. See https://developers.deepgram.com/docs/the-deepgram-model-improvement-partnership-program#want-to-opt-out  This will only be used if you are using your own Deepgram API key.  @default false
    #[serde(rename = "mipOptOut", skip_serializing_if = "Option::is_none")]
    pub mip_opt_out: Option<bool>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
}

impl FallbackDeepgramVoice {
    pub fn new(provider: Provider, voice_id: VoiceId) -> FallbackDeepgramVoice {
        FallbackDeepgramVoice {
            provider,
            voice_id,
            model: None,
            mip_opt_out: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "deepgram")]
    Deepgram,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Deepgram
    }
}
/// This is the provider-specific ID that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceId {
    #[serde(rename = "asteria")]
    Asteria,
    #[serde(rename = "luna")]
    Luna,
    #[serde(rename = "stella")]
    Stella,
    #[serde(rename = "athena")]
    Athena,
    #[serde(rename = "hera")]
    Hera,
    #[serde(rename = "orion")]
    Orion,
    #[serde(rename = "arcas")]
    Arcas,
    #[serde(rename = "perseus")]
    Perseus,
    #[serde(rename = "angus")]
    Angus,
    #[serde(rename = "orpheus")]
    Orpheus,
    #[serde(rename = "helios")]
    Helios,
    #[serde(rename = "zeus")]
    Zeus,
    #[serde(rename = "thalia")]
    Thalia,
    #[serde(rename = "andromeda")]
    Andromeda,
    #[serde(rename = "helena")]
    Helena,
    #[serde(rename = "apollo")]
    Apollo,
    #[serde(rename = "arcas-2")]
    Arcas2,
    #[serde(rename = "aries")]
    Aries,
    #[serde(rename = "amalthea")]
    Amalthea,
    #[serde(rename = "andromeda-2")]
    Andromeda2,
    #[serde(rename = "apollo-2")]
    Apollo2,
    #[serde(rename = "arcas-3")]
    Arcas3,
    #[serde(rename = "aries-2")]
    Aries2,
    #[serde(rename = "asteria-2")]
    Asteria2,
    #[serde(rename = "athena-2")]
    Athena2,
    #[serde(rename = "atlas")]
    Atlas,
    #[serde(rename = "aurora")]
    Aurora,
    #[serde(rename = "callista")]
    Callista,
    #[serde(rename = "cora")]
    Cora,
    #[serde(rename = "cordelia")]
    Cordelia,
    #[serde(rename = "delia")]
    Delia,
    #[serde(rename = "draco")]
    Draco,
    #[serde(rename = "electra")]
    Electra,
    #[serde(rename = "harmonia")]
    Harmonia,
    #[serde(rename = "helena-2")]
    Helena2,
    #[serde(rename = "hera-2")]
    Hera2,
    #[serde(rename = "hermes")]
    Hermes,
    #[serde(rename = "hyperion")]
    Hyperion,
    #[serde(rename = "iris")]
    Iris,
    #[serde(rename = "janus")]
    Janus,
    #[serde(rename = "juno")]
    Juno,
    #[serde(rename = "jupiter")]
    Jupiter,
    #[serde(rename = "luna-2")]
    Luna2,
    #[serde(rename = "mars")]
    Mars,
    #[serde(rename = "minerva")]
    Minerva,
    #[serde(rename = "neptune")]
    Neptune,
    #[serde(rename = "odysseus")]
    Odysseus,
    #[serde(rename = "ophelia")]
    Ophelia,
    #[serde(rename = "orion-2")]
    Orion2,
    #[serde(rename = "orpheus-2")]
    Orpheus2,
    #[serde(rename = "pandora")]
    Pandora,
    #[serde(rename = "phoebe")]
    Phoebe,
    #[serde(rename = "pluto")]
    Pluto,
    #[serde(rename = "saturn")]
    Saturn,
    #[serde(rename = "selene")]
    Selene,
    #[serde(rename = "thalia-2")]
    Thalia2,
    #[serde(rename = "theia")]
    Theia,
    #[serde(rename = "vesta")]
    Vesta,
    #[serde(rename = "zeus-2")]
    Zeus2,
}

impl Default for VoiceId {
    fn default() -> VoiceId {
        Self::Asteria
    }
}
/// This is the model that will be used. Defaults to 'aura-2' when not specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "aura")]
    Aura,
    #[serde(rename = "aura-2")]
    Aura2,
}

impl Default for Model {
    fn default() -> Model {
        Self::Aura
    }
}

