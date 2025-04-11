/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StopSpeakingPlan {
    /// This is the number of words that the customer has to say before the assistant will stop talking.  Words like \"stop\", \"actually\", \"no\", etc. will always interrupt immediately regardless of this value.  Words like \"okay\", \"yeah\", \"right\" will never interrupt.  When set to 0, `voiceSeconds` is used in addition to the transcriptions to determine the customer has started speaking.  Defaults to 0.  @default 0
    #[serde(rename = "numWords", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_words: Option<Option<f64>>,
    /// This is the seconds customer has to speak before the assistant stops talking. This uses the VAD (Voice Activity Detection) spike to determine if the customer has started speaking.  Considerations: - A lower value might be more responsive but could potentially pick up non-speech sounds. - A higher value reduces false positives but might slightly delay the detection of speech onset.  This is only used if `numWords` is set to 0.  Defaults to 0.2  @default 0.2
    #[serde(rename = "voiceSeconds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_seconds: Option<Option<f64>>,
    /// This is the seconds to wait before the assistant will start talking again after being interrupted.  Defaults to 1.  @default 1
    #[serde(rename = "backoffSeconds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub backoff_seconds: Option<Option<f64>>,
    /// These are the phrases that will never interrupt the assistant, even if numWords threshold is met. These are typically acknowledgement or backchanneling phrases.
    #[serde(rename = "acknowledgementPhrases", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub acknowledgement_phrases: Option<Option<Vec<String>>>,
    /// These are the phrases that will always interrupt the assistant immediately, regardless of numWords. These are typically phrases indicating disagreement or desire to stop.
    #[serde(rename = "interruptionPhrases", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interruption_phrases: Option<Option<Vec<String>>>,
}

impl StopSpeakingPlan {
    pub fn new() -> StopSpeakingPlan {
        StopSpeakingPlan {
            num_words: None,
            voice_seconds: None,
            backoff_seconds: None,
            acknowledgement_phrases: None,
            interruption_phrases: None,
        }
    }
}

