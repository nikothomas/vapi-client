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
pub struct ElevenLabsTranscriber {
    /// This is the transcription provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the model that will be used for the transcription.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
}

impl ElevenLabsTranscriber {
    pub fn new(provider: Provider) -> ElevenLabsTranscriber {
        ElevenLabsTranscriber {
            provider,
            model: None,
            language: None,
        }
    }
}
/// This is the transcription provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "11labs")]
    Variant11labs,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Variant11labs
    }
}
/// This is the model that will be used for the transcription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "scribe_v1")]
    ScribeV1,
}

impl Default for Model {
    fn default() -> Model {
        Self::ScribeV1
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "aa")]
    Aa,
    #[serde(rename = "ab")]
    Ab,
    #[serde(rename = "ae")]
    Ae,
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "ak")]
    Ak,
    #[serde(rename = "am")]
    Am,
    #[serde(rename = "an")]
    An,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "as")]
    As,
    #[serde(rename = "av")]
    Av,
    #[serde(rename = "ay")]
    Ay,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "ba")]
    Ba,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bh")]
    Bh,
    #[serde(rename = "bi")]
    Bi,
    #[serde(rename = "bm")]
    Bm,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bo")]
    Bo,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "ce")]
    Ce,
    #[serde(rename = "ch")]
    Ch,
    #[serde(rename = "co")]
    Co,
    #[serde(rename = "cr")]
    Cr,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "cu")]
    Cu,
    #[serde(rename = "cv")]
    Cv,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "dv")]
    Dv,
    #[serde(rename = "dz")]
    Dz,
    #[serde(rename = "ee")]
    Ee,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "eo")]
    Eo,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "ff")]
    Ff,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fj")]
    Fj,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fy")]
    Fy,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "gd")]
    Gd,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "gn")]
    Gn,
    #[serde(rename = "gu")]
    Gu,
    #[serde(rename = "gv")]
    Gv,
    #[serde(rename = "ha")]
    Ha,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "ho")]
    Ho,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "hy")]
    Hy,
    #[serde(rename = "hz")]
    Hz,
    #[serde(rename = "ia")]
    Ia,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "ie")]
    Ie,
    #[serde(rename = "ig")]
    Ig,
    #[serde(rename = "ii")]
    Ii,
    #[serde(rename = "ik")]
    Ik,
    #[serde(rename = "io")]
    Io,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "iu")]
    Iu,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "jv")]
    Jv,
    #[serde(rename = "ka")]
    Ka,
    #[serde(rename = "kg")]
    Kg,
    #[serde(rename = "ki")]
    Ki,
    #[serde(rename = "kj")]
    Kj,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "kl")]
    Kl,
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "kr")]
    Kr,
    #[serde(rename = "ks")]
    Ks,
    #[serde(rename = "ku")]
    Ku,
    #[serde(rename = "kv")]
    Kv,
    #[serde(rename = "kw")]
    Kw,
    #[serde(rename = "ky")]
    Ky,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "lg")]
    Lg,
    #[serde(rename = "li")]
    Li,
    #[serde(rename = "ln")]
    Ln,
    #[serde(rename = "lo")]
    Lo,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lu")]
    Lu,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "mg")]
    Mg,
    #[serde(rename = "mh")]
    Mh,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "mn")]
    Mn,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "my")]
    My,
    #[serde(rename = "na")]
    Na,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "nd")]
    Nd,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "ng")]
    Ng,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nn")]
    Nn,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "nr")]
    Nr,
    #[serde(rename = "nv")]
    Nv,
    #[serde(rename = "ny")]
    Ny,
    #[serde(rename = "oc")]
    Oc,
    #[serde(rename = "oj")]
    Oj,
    #[serde(rename = "om")]
    Om,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "os")]
    Os,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "pi")]
    Pi,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "ps")]
    Ps,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "qu")]
    Qu,
    #[serde(rename = "rm")]
    Rm,
    #[serde(rename = "rn")]
    Rn,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "rw")]
    Rw,
    #[serde(rename = "sa")]
    Sa,
    #[serde(rename = "sc")]
    Sc,
    #[serde(rename = "sd")]
    Sd,
    #[serde(rename = "se")]
    Se,
    #[serde(rename = "sg")]
    Sg,
    #[serde(rename = "si")]
    Si,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "so")]
    So,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "ss")]
    Ss,
    #[serde(rename = "st")]
    St,
    #[serde(rename = "su")]
    Su,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "te")]
    Te,
    #[serde(rename = "tg")]
    Tg,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "ti")]
    Ti,
    #[serde(rename = "tk")]
    Tk,
    #[serde(rename = "tl")]
    Tl,
    #[serde(rename = "tn")]
    Tn,
    #[serde(rename = "to")]
    To,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "ts")]
    Ts,
    #[serde(rename = "tt")]
    Tt,
    #[serde(rename = "tw")]
    Tw,
    #[serde(rename = "ty")]
    Ty,
    #[serde(rename = "ug")]
    Ug,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "ve")]
    Ve,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "vo")]
    Vo,
    #[serde(rename = "wa")]
    Wa,
    #[serde(rename = "wo")]
    Wo,
    #[serde(rename = "xh")]
    Xh,
    #[serde(rename = "yi")]
    Yi,
    #[serde(rename = "yue")]
    Yue,
    #[serde(rename = "yo")]
    Yo,
    #[serde(rename = "za")]
    Za,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "zu")]
    Zu,
}

impl Default for Language {
    fn default() -> Language {
        Self::Aa
    }
}

