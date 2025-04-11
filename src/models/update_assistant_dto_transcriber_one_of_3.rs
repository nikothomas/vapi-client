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
pub struct UpdateAssistantDtoTranscriberOneOf3 {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<models::DeepgramTranscriberModel>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<models::DeepgramTranscriberLanguage>,
    /// This will be use smart format option provided by Deepgram. It's default disabled because it can sometimes format numbers as times but it's getting better.
    #[serde(rename = "smartFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub smart_format: Option<Option<bool>>,
    /// This automatically switches the transcriber's language when the customer's language changes. Defaults to false.  Usage: - If your customers switch languages mid-call, you can set this to true.  Note: - To detect language changes, Vapi uses a custom trained model. Languages supported (X = limited support):   1. Arabic   2. Bengali   3. Cantonese   4. Chinese   5. Chinese Simplified (X)   6. Chinese Traditional (X)   7. English   8. Farsi (X)   9. French   10. German   11. Haitian Creole (X)   12. Hindi   13. Italian   14. Japanese   15. Korean   16. Portuguese   17. Russian   18. Spanish   19. Thai   20. Urdu   21. Vietnamese - To receive `language-change-detected` webhook events, add it to `assistant.serverMessages`.  @default false
    #[serde(rename = "codeSwitchingEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code_switching_enabled: Option<Option<bool>>,
    /// If set to true, this will add mip_opt_out=true as a query parameter of all API requests. See https://developers.deepgram.com/docs/the-deepgram-model-improvement-partnership-program#want-to-opt-out  This will only be used if you are using your own Deepgram API key.  @default false
    #[serde(rename = "mipOptOut", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mip_opt_out: Option<Option<bool>>,
    /// If set to true, this will cause deepgram to convert spoken numbers to literal numerals. For example, \"my phone number is nine-seven-two...\" would become \"my phone number is 972...\"  @default false
    #[serde(rename = "numerals", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub numerals: Option<Option<bool>>,
    /// These keywords are passed to the transcription model to help it pick up use-case specific words. Anything that may not be a common word, like your company name, should be added here.
    #[serde(rename = "keywords", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Option<Vec<String>>>,
    /// Keyterm Prompting allows you improve Keyword Recall Rate (KRR) for important keyterms or phrases up to 90%.
    #[serde(rename = "keyterm", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub keyterm: Option<Option<Vec<String>>>,
    /// This is the timeout after which Deepgram will send transcription on user silence. You can read in-depth documentation here: https://developers.deepgram.com/docs/endpointing.  Here are the most important bits: - Defaults to 10. This is recommended for most use cases to optimize for latency. - 10 can cause some missing transcriptions since because of the shorter context. This mostly happens for one-word utterances. For those uses cases, it's recommended to try 300. It will add a bit of latency but the quality and reliability of the experience will be better. - If neither 10 nor 300 work, contact support@vapi.ai and we'll find another solution.  @default 10
    #[serde(rename = "endpointing", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub endpointing: Option<Option<f64>>,
    #[serde(rename = "fallbackPlan", skip_serializing_if = "Option::is_none")]
    pub fallback_plan: Option<models::FallbackTranscriberPlan>,
    #[serde(rename = "provider")]
    pub provider: Provider,
}

impl UpdateAssistantDtoTranscriberOneOf3 {
    pub fn new(provider: Provider) -> UpdateAssistantDtoTranscriberOneOf3 {
        UpdateAssistantDtoTranscriberOneOf3 {
            model: None,
            language: None,
            smart_format: None,
            code_switching_enabled: None,
            mip_opt_out: None,
            numerals: None,
            keywords: None,
            keyterm: None,
            endpointing: None,
            fallback_plan: None,
            provider,
        }
    }
}
/// 
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

