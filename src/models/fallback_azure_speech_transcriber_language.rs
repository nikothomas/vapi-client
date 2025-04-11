/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FallbackAzureSpeechTranscriberLanguage : This is the language that will be set for the transcription. The list of languages Azure supports can be found here: https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=stt
/// This is the language that will be set for the transcription. The list of languages Azure supports can be found here: https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=stt
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FallbackAzureSpeechTranscriberLanguage {
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

impl std::fmt::Display for FallbackAzureSpeechTranscriberLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AfZa => write!(f, "af-ZA"),
            Self::AmEt => write!(f, "am-ET"),
            Self::ArAe => write!(f, "ar-AE"),
            Self::ArBh => write!(f, "ar-BH"),
            Self::ArDz => write!(f, "ar-DZ"),
            Self::ArEg => write!(f, "ar-EG"),
            Self::ArIl => write!(f, "ar-IL"),
            Self::ArIq => write!(f, "ar-IQ"),
            Self::ArJo => write!(f, "ar-JO"),
            Self::ArKw => write!(f, "ar-KW"),
            Self::ArLb => write!(f, "ar-LB"),
            Self::ArLy => write!(f, "ar-LY"),
            Self::ArMa => write!(f, "ar-MA"),
            Self::ArOm => write!(f, "ar-OM"),
            Self::ArPs => write!(f, "ar-PS"),
            Self::ArQa => write!(f, "ar-QA"),
            Self::ArSa => write!(f, "ar-SA"),
            Self::ArSy => write!(f, "ar-SY"),
            Self::ArTn => write!(f, "ar-TN"),
            Self::ArYe => write!(f, "ar-YE"),
            Self::AzAz => write!(f, "az-AZ"),
            Self::BgBg => write!(f, "bg-BG"),
            Self::BnIn => write!(f, "bn-IN"),
            Self::BsBa => write!(f, "bs-BA"),
            Self::CaEs => write!(f, "ca-ES"),
            Self::CsCz => write!(f, "cs-CZ"),
            Self::CyGb => write!(f, "cy-GB"),
            Self::DaDk => write!(f, "da-DK"),
            Self::DeAt => write!(f, "de-AT"),
            Self::DeCh => write!(f, "de-CH"),
            Self::DeDe => write!(f, "de-DE"),
            Self::ElGr => write!(f, "el-GR"),
            Self::EnAu => write!(f, "en-AU"),
            Self::EnCa => write!(f, "en-CA"),
            Self::EnGb => write!(f, "en-GB"),
            Self::EnGh => write!(f, "en-GH"),
            Self::EnHk => write!(f, "en-HK"),
            Self::EnIe => write!(f, "en-IE"),
            Self::EnIn => write!(f, "en-IN"),
            Self::EnKe => write!(f, "en-KE"),
            Self::EnNg => write!(f, "en-NG"),
            Self::EnNz => write!(f, "en-NZ"),
            Self::EnPh => write!(f, "en-PH"),
            Self::EnSg => write!(f, "en-SG"),
            Self::EnTz => write!(f, "en-TZ"),
            Self::EnUs => write!(f, "en-US"),
            Self::EnZa => write!(f, "en-ZA"),
            Self::EsAr => write!(f, "es-AR"),
            Self::EsBo => write!(f, "es-BO"),
            Self::EsCl => write!(f, "es-CL"),
            Self::EsCo => write!(f, "es-CO"),
            Self::EsCr => write!(f, "es-CR"),
            Self::EsCu => write!(f, "es-CU"),
            Self::EsDo => write!(f, "es-DO"),
            Self::EsEc => write!(f, "es-EC"),
            Self::EsEs => write!(f, "es-ES"),
            Self::EsGq => write!(f, "es-GQ"),
            Self::EsGt => write!(f, "es-GT"),
            Self::EsHn => write!(f, "es-HN"),
            Self::EsMx => write!(f, "es-MX"),
            Self::EsNi => write!(f, "es-NI"),
            Self::EsPa => write!(f, "es-PA"),
            Self::EsPe => write!(f, "es-PE"),
            Self::EsPr => write!(f, "es-PR"),
            Self::EsPy => write!(f, "es-PY"),
            Self::EsSv => write!(f, "es-SV"),
            Self::EsUs => write!(f, "es-US"),
            Self::EsUy => write!(f, "es-UY"),
            Self::EsVe => write!(f, "es-VE"),
            Self::EtEe => write!(f, "et-EE"),
            Self::EuEs => write!(f, "eu-ES"),
            Self::FaIr => write!(f, "fa-IR"),
            Self::FiFi => write!(f, "fi-FI"),
            Self::FilPh => write!(f, "fil-PH"),
            Self::FrBe => write!(f, "fr-BE"),
            Self::FrCa => write!(f, "fr-CA"),
            Self::FrCh => write!(f, "fr-CH"),
            Self::FrFr => write!(f, "fr-FR"),
            Self::GaIe => write!(f, "ga-IE"),
            Self::GlEs => write!(f, "gl-ES"),
            Self::GuIn => write!(f, "gu-IN"),
            Self::HeIl => write!(f, "he-IL"),
            Self::HiIn => write!(f, "hi-IN"),
            Self::HrHr => write!(f, "hr-HR"),
            Self::HuHu => write!(f, "hu-HU"),
            Self::HyAm => write!(f, "hy-AM"),
            Self::IdId => write!(f, "id-ID"),
            Self::IsIs => write!(f, "is-IS"),
            Self::ItCh => write!(f, "it-CH"),
            Self::ItIt => write!(f, "it-IT"),
            Self::JaJp => write!(f, "ja-JP"),
            Self::JvId => write!(f, "jv-ID"),
            Self::KaGe => write!(f, "ka-GE"),
            Self::KkKz => write!(f, "kk-KZ"),
            Self::KmKh => write!(f, "km-KH"),
            Self::KnIn => write!(f, "kn-IN"),
            Self::KoKr => write!(f, "ko-KR"),
            Self::LoLa => write!(f, "lo-LA"),
            Self::LtLt => write!(f, "lt-LT"),
            Self::LvLv => write!(f, "lv-LV"),
            Self::MkMk => write!(f, "mk-MK"),
            Self::MlIn => write!(f, "ml-IN"),
            Self::MnMn => write!(f, "mn-MN"),
            Self::MrIn => write!(f, "mr-IN"),
            Self::MsMy => write!(f, "ms-MY"),
            Self::MtMt => write!(f, "mt-MT"),
            Self::MyMm => write!(f, "my-MM"),
            Self::NbNo => write!(f, "nb-NO"),
            Self::NeNp => write!(f, "ne-NP"),
            Self::NlBe => write!(f, "nl-BE"),
            Self::NlNl => write!(f, "nl-NL"),
            Self::PaIn => write!(f, "pa-IN"),
            Self::PlPl => write!(f, "pl-PL"),
            Self::PsAf => write!(f, "ps-AF"),
            Self::PtBr => write!(f, "pt-BR"),
            Self::PtPt => write!(f, "pt-PT"),
            Self::RoRo => write!(f, "ro-RO"),
            Self::RuRu => write!(f, "ru-RU"),
            Self::SiLk => write!(f, "si-LK"),
            Self::SkSk => write!(f, "sk-SK"),
            Self::SlSi => write!(f, "sl-SI"),
            Self::SoSo => write!(f, "so-SO"),
            Self::SqAl => write!(f, "sq-AL"),
            Self::SrRs => write!(f, "sr-RS"),
            Self::SvSe => write!(f, "sv-SE"),
            Self::SwKe => write!(f, "sw-KE"),
            Self::SwTz => write!(f, "sw-TZ"),
            Self::TaIn => write!(f, "ta-IN"),
            Self::TeIn => write!(f, "te-IN"),
            Self::ThTh => write!(f, "th-TH"),
            Self::TrTr => write!(f, "tr-TR"),
            Self::UkUa => write!(f, "uk-UA"),
            Self::UrIn => write!(f, "ur-IN"),
            Self::UzUz => write!(f, "uz-UZ"),
            Self::ViVn => write!(f, "vi-VN"),
            Self::WuuCn => write!(f, "wuu-CN"),
            Self::YueCn => write!(f, "yue-CN"),
            Self::ZhCn => write!(f, "zh-CN"),
            Self::ZhCnShandong => write!(f, "zh-CN-shandong"),
            Self::ZhCnSichuan => write!(f, "zh-CN-sichuan"),
            Self::ZhHk => write!(f, "zh-HK"),
            Self::ZhTw => write!(f, "zh-TW"),
            Self::ZuZa => write!(f, "zu-ZA"),
        }
    }
}

impl Default for FallbackAzureSpeechTranscriberLanguage {
    fn default() -> FallbackAzureSpeechTranscriberLanguage {
        Self::AfZa
    }
}

