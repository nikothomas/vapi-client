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
pub struct FallbackPlayHtVoice {
    /// This is the voice provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "voiceId")]
    pub voice_id: models::PlayHtVoiceVoiceId,
    /// This is the speed multiplier that will be used.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// A floating point number between 0, exclusive, and 2, inclusive. If equal to null or not provided, the model's default temperature will be used. The temperature parameter controls variance. Lower temperatures result in more predictable results, higher temperatures allow each run to vary more, so the voice may sound less like the baseline voice.
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// An emotion to be applied to the speech.
    #[serde(rename = "emotion", skip_serializing_if = "Option::is_none")]
    pub emotion: Option<Emotion>,
    /// A number between 1 and 6. Use lower numbers to reduce how unique your chosen voice will be compared to other voices.
    #[serde(rename = "voiceGuidance", skip_serializing_if = "Option::is_none")]
    pub voice_guidance: Option<f64>,
    /// A number between 1 and 30. Use lower numbers to to reduce how strong your chosen emotion will be. Higher numbers will create a very emotional performance.
    #[serde(rename = "styleGuidance", skip_serializing_if = "Option::is_none")]
    pub style_guidance: Option<f64>,
    /// A number between 1 and 2. This number influences how closely the generated speech adheres to the input text. Use lower values to create more fluid speech, but with a higher chance of deviating from the input text. Higher numbers will make the generated speech more accurate to the input text, ensuring that the words spoken align closely with the provided text.
    #[serde(rename = "textGuidance", skip_serializing_if = "Option::is_none")]
    pub text_guidance: Option<f64>,
    /// Playht voice model/engine to use.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    /// The language to use for the speech.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// This is the plan for chunking the model output before it is sent to the voice provider.
    #[serde(rename = "chunkPlan", skip_serializing_if = "Option::is_none")]
    pub chunk_plan: Option<models::ChunkPlan>,
}

impl FallbackPlayHtVoice {
    pub fn new(provider: Provider, voice_id: models::PlayHtVoiceVoiceId) -> FallbackPlayHtVoice {
        FallbackPlayHtVoice {
            provider,
            voice_id,
            speed: None,
            temperature: None,
            emotion: None,
            voice_guidance: None,
            style_guidance: None,
            text_guidance: None,
            model: None,
            language: None,
            chunk_plan: None,
        }
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Provider {
    #[serde(rename = "playht")]
    Playht,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Playht
    }
}
/// An emotion to be applied to the speech.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Emotion {
    #[serde(rename = "female_happy")]
    FemaleHappy,
    #[serde(rename = "female_sad")]
    FemaleSad,
    #[serde(rename = "female_angry")]
    FemaleAngry,
    #[serde(rename = "female_fearful")]
    FemaleFearful,
    #[serde(rename = "female_disgust")]
    FemaleDisgust,
    #[serde(rename = "female_surprised")]
    FemaleSurprised,
    #[serde(rename = "male_happy")]
    MaleHappy,
    #[serde(rename = "male_sad")]
    MaleSad,
    #[serde(rename = "male_angry")]
    MaleAngry,
    #[serde(rename = "male_fearful")]
    MaleFearful,
    #[serde(rename = "male_disgust")]
    MaleDisgust,
    #[serde(rename = "male_surprised")]
    MaleSurprised,
}

impl Default for Emotion {
    fn default() -> Emotion {
        Self::FemaleHappy
    }
}
/// Playht voice model/engine to use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Model {
    #[serde(rename = "PlayHT2.0")]
    PlayHt2Period0,
    #[serde(rename = "PlayHT2.0-turbo")]
    PlayHt2Period0Turbo,
    #[serde(rename = "Play3.0-mini")]
    Play3Period0Mini,
    #[serde(rename = "PlayDialog")]
    PlayDialog,
}

impl Default for Model {
    fn default() -> Model {
        Self::PlayHt2Period0
    }
}
/// The language to use for the speech.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Language {
    #[serde(rename = "afrikaans")]
    Afrikaans,
    #[serde(rename = "albanian")]
    Albanian,
    #[serde(rename = "amharic")]
    Amharic,
    #[serde(rename = "arabic")]
    Arabic,
    #[serde(rename = "bengali")]
    Bengali,
    #[serde(rename = "bulgarian")]
    Bulgarian,
    #[serde(rename = "catalan")]
    Catalan,
    #[serde(rename = "croatian")]
    Croatian,
    #[serde(rename = "czech")]
    Czech,
    #[serde(rename = "danish")]
    Danish,
    #[serde(rename = "dutch")]
    Dutch,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "galician")]
    Galician,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "greek")]
    Greek,
    #[serde(rename = "hebrew")]
    Hebrew,
    #[serde(rename = "hindi")]
    Hindi,
    #[serde(rename = "hungarian")]
    Hungarian,
    #[serde(rename = "indonesian")]
    Indonesian,
    #[serde(rename = "italian")]
    Italian,
    #[serde(rename = "japanese")]
    Japanese,
    #[serde(rename = "korean")]
    Korean,
    #[serde(rename = "malay")]
    Malay,
    #[serde(rename = "mandarin")]
    Mandarin,
    #[serde(rename = "polish")]
    Polish,
    #[serde(rename = "portuguese")]
    Portuguese,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "serbian")]
    Serbian,
    #[serde(rename = "spanish")]
    Spanish,
    #[serde(rename = "swedish")]
    Swedish,
    #[serde(rename = "tagalog")]
    Tagalog,
    #[serde(rename = "thai")]
    Thai,
    #[serde(rename = "turkish")]
    Turkish,
    #[serde(rename = "ukrainian")]
    Ukrainian,
    #[serde(rename = "urdu")]
    Urdu,
    #[serde(rename = "xhosa")]
    Xhosa,
}

impl Default for Language {
    fn default() -> Language {
        Self::Afrikaans
    }
}
