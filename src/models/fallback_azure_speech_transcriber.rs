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
pub struct FallbackAzureSpeechTranscriber {
    /// This is the transcription provider that will be used.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is the language that will be set for the transcription. The list of languages Azure supports can be found here: https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=stt
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
}

impl FallbackAzureSpeechTranscriber {
    pub fn new(provider: Provider) -> FallbackAzureSpeechTranscriber {
        FallbackAzureSpeechTranscriber {
            provider,
            language: None,
        }
    }
}
/// This is the transcription provider that will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "azure")]
    Azure,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Azure
    }
}
/// This is the language that will be set for the transcription. The list of languages Azure supports can be found here: https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=stt
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "af-ZA")]
    AfZa,
    #[serde(rename = "am-ET")]
    AmEt,
    #[serde(rename = "ar-AE")]
    ArAe,
    #[serde(rename = "ar-BH")]
    ArBh,
    #[serde(rename = "ar-DZ")]
    ArDz,
    #[serde(rename = "ar-EG")]
    ArEg,
    #[serde(rename = "ar-IL")]
    ArIl,
    #[serde(rename = "ar-IQ")]
    ArIq,
    #[serde(rename = "ar-JO")]
    ArJo,
    #[serde(rename = "ar-KW")]
    ArKw,
    #[serde(rename = "ar-LB")]
    ArLb,
    #[serde(rename = "ar-LY")]
    ArLy,
    #[serde(rename = "ar-MA")]
    ArMa,
    #[serde(rename = "ar-OM")]
    ArOm,
    #[serde(rename = "ar-PS")]
    ArPs,
    #[serde(rename = "ar-QA")]
    ArQa,
    #[serde(rename = "ar-SA")]
    ArSa,
    #[serde(rename = "ar-SY")]
    ArSy,
    #[serde(rename = "ar-TN")]
    ArTn,
    #[serde(rename = "ar-YE")]
    ArYe,
    #[serde(rename = "az-AZ")]
    AzAz,
    #[serde(rename = "bg-BG")]
    BgBg,
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "bs-BA")]
    BsBa,
    #[serde(rename = "ca-ES")]
    CaEs,
    #[serde(rename = "cs-CZ")]
    CsCz,
    #[serde(rename = "cy-GB")]
    CyGb,
    #[serde(rename = "da-DK")]
    DaDk,
    #[serde(rename = "de-AT")]
    DeAt,
    #[serde(rename = "de-CH")]
    DeCh,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "el-GR")]
    ElGr,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-CA")]
    EnCa,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-GH")]
    EnGh,
    #[serde(rename = "en-HK")]
    EnHk,
    #[serde(rename = "en-IE")]
    EnIe,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "en-KE")]
    EnKe,
    #[serde(rename = "en-NG")]
    EnNg,
    #[serde(rename = "en-NZ")]
    EnNz,
    #[serde(rename = "en-PH")]
    EnPh,
    #[serde(rename = "en-SG")]
    EnSg,
    #[serde(rename = "en-TZ")]
    EnTz,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "en-ZA")]
    EnZa,
    #[serde(rename = "es-AR")]
    EsAr,
    #[serde(rename = "es-BO")]
    EsBo,
    #[serde(rename = "es-CL")]
    EsCl,
    #[serde(rename = "es-CO")]
    EsCo,
    #[serde(rename = "es-CR")]
    EsCr,
    #[serde(rename = "es-CU")]
    EsCu,
    #[serde(rename = "es-DO")]
    EsDo,
    #[serde(rename = "es-EC")]
    EsEc,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-GQ")]
    EsGq,
    #[serde(rename = "es-GT")]
    EsGt,
    #[serde(rename = "es-HN")]
    EsHn,
    #[serde(rename = "es-MX")]
    EsMx,
    #[serde(rename = "es-NI")]
    EsNi,
    #[serde(rename = "es-PA")]
    EsPa,
    #[serde(rename = "es-PE")]
    EsPe,
    #[serde(rename = "es-PR")]
    EsPr,
    #[serde(rename = "es-PY")]
    EsPy,
    #[serde(rename = "es-SV")]
    EsSv,
    #[serde(rename = "es-US")]
    EsUs,
    #[serde(rename = "es-UY")]
    EsUy,
    #[serde(rename = "es-VE")]
    EsVe,
    #[serde(rename = "et-EE")]
    EtEe,
    #[serde(rename = "eu-ES")]
    EuEs,
    #[serde(rename = "fa-IR")]
    FaIr,
    #[serde(rename = "fi-FI")]
    FiFi,
    #[serde(rename = "fil-PH")]
    FilPh,
    #[serde(rename = "fr-BE")]
    FrBe,
    #[serde(rename = "fr-CA")]
    FrCa,
    #[serde(rename = "fr-CH")]
    FrCh,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "ga-IE")]
    GaIe,
    #[serde(rename = "gl-ES")]
    GlEs,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "he-IL")]
    HeIl,
    #[serde(rename = "hi-IN")]
    HiIn,
    #[serde(rename = "hr-HR")]
    HrHr,
    #[serde(rename = "hu-HU")]
    HuHu,
    #[serde(rename = "hy-AM")]
    HyAm,
    #[serde(rename = "id-ID")]
    IdId,
    #[serde(rename = "is-IS")]
    IsIs,
    #[serde(rename = "it-CH")]
    ItCh,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "ja-JP")]
    JaJp,
    #[serde(rename = "jv-ID")]
    JvId,
    #[serde(rename = "ka-GE")]
    KaGe,
    #[serde(rename = "kk-KZ")]
    KkKz,
    #[serde(rename = "km-KH")]
    KmKh,
    #[serde(rename = "kn-IN")]
    KnIn,
    #[serde(rename = "ko-KR")]
    KoKr,
    #[serde(rename = "lo-LA")]
    LoLa,
    #[serde(rename = "lt-LT")]
    LtLt,
    #[serde(rename = "lv-LV")]
    LvLv,
    #[serde(rename = "mk-MK")]
    MkMk,
    #[serde(rename = "ml-IN")]
    MlIn,
    #[serde(rename = "mn-MN")]
    MnMn,
    #[serde(rename = "mr-IN")]
    MrIn,
    #[serde(rename = "ms-MY")]
    MsMy,
    #[serde(rename = "mt-MT")]
    MtMt,
    #[serde(rename = "my-MM")]
    MyMm,
    #[serde(rename = "nb-NO")]
    NbNo,
    #[serde(rename = "ne-NP")]
    NeNp,
    #[serde(rename = "nl-BE")]
    NlBe,
    #[serde(rename = "nl-NL")]
    NlNl,
    #[serde(rename = "pa-IN")]
    PaIn,
    #[serde(rename = "pl-PL")]
    PlPl,
    #[serde(rename = "ps-AF")]
    PsAf,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "ro-RO")]
    RoRo,
    #[serde(rename = "ru-RU")]
    RuRu,
    #[serde(rename = "si-LK")]
    SiLk,
    #[serde(rename = "sk-SK")]
    SkSk,
    #[serde(rename = "sl-SI")]
    SlSi,
    #[serde(rename = "so-SO")]
    SoSo,
    #[serde(rename = "sq-AL")]
    SqAl,
    #[serde(rename = "sr-RS")]
    SrRs,
    #[serde(rename = "sv-SE")]
    SvSe,
    #[serde(rename = "sw-KE")]
    SwKe,
    #[serde(rename = "sw-TZ")]
    SwTz,
    #[serde(rename = "ta-IN")]
    TaIn,
    #[serde(rename = "te-IN")]
    TeIn,
    #[serde(rename = "th-TH")]
    ThTh,
    #[serde(rename = "tr-TR")]
    TrTr,
    #[serde(rename = "uk-UA")]
    UkUa,
    #[serde(rename = "ur-IN")]
    UrIn,
    #[serde(rename = "uz-UZ")]
    UzUz,
    #[serde(rename = "vi-VN")]
    ViVn,
    #[serde(rename = "wuu-CN")]
    WuuCn,
    #[serde(rename = "yue-CN")]
    YueCn,
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "zh-CN-shandong")]
    ZhCnShandong,
    #[serde(rename = "zh-CN-sichuan")]
    ZhCnSichuan,
    #[serde(rename = "zh-HK")]
    ZhHk,
    #[serde(rename = "zh-TW")]
    ZhTw,
    #[serde(rename = "zu-ZA")]
    ZuZa,
}

impl Default for Language {
    fn default() -> Language {
        Self::AfZa
    }
}

