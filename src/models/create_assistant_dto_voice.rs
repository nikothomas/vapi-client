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

/// CreateAssistantDtoVoice : These are the options for the assistant's voice.
/// These are the options for the assistant's voice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantDtoVoice {
    AzureVoice(models::AzureVoice),
    CartesiaVoice(models::CartesiaVoice),
    CustomVoice(models::CustomVoice),
    DeepgramVoice(models::DeepgramVoice),
    ElevenLabsVoice(models::ElevenLabsVoice),
    LmntVoice(models::LmntVoice),
    NeetsVoice(models::NeetsVoice),
    OpenAiVoice(models::OpenAiVoice),
    PlayHtVoice(models::PlayHtVoice),
    RimeAiVoice(models::RimeAiVoice),
    SmallestAiVoice(models::SmallestAiVoice),
    TavusVoice(models::TavusVoice),
}

impl Default for CreateAssistantDtoVoice {
    fn default() -> Self {
        Self::AzureVoice(Default::default())
    }
}
/// This is the voice provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "tavus")]
    Tavus,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Tavus
    }
}
/// Smallest AI voice model to use. Defaults to 'lightning' when not specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "lightning")]
    Lightning,
}

impl Default for Model {
    fn default() -> Model {
        Self::Lightning
    }
}
/// The language to use for the speech.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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
/// An emotion to be applied to the speech.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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
