# DeepgramTranscriber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<[**models::DeepgramTranscriberModel**](DeepgramTranscriberModel.md)> |  | [optional]
**language** | Option<[**models::DeepgramTranscriberLanguage**](DeepgramTranscriberLanguage.md)> |  | [optional]
**smart_format** | Option<**bool**> | This will be use smart format option provided by Deepgram. It's default disabled because it can sometimes format numbers as times but it's getting better. | [optional]
**code_switching_enabled** | Option<**bool**> | This automatically switches the transcriber's language when the customer's language changes. Defaults to false.  Usage: - If your customers switch languages mid-call, you can set this to true.  Note: - To detect language changes, Vapi uses a custom trained model. Languages supported (X = limited support):   1. Arabic   2. Bengali   3. Cantonese   4. Chinese   5. Chinese Simplified (X)   6. Chinese Traditional (X)   7. English   8. Farsi (X)   9. French   10. German   11. Haitian Creole (X)   12. Hindi   13. Italian   14. Japanese   15. Korean   16. Portuguese   17. Russian   18. Spanish   19. Thai   20. Urdu   21. Vietnamese - To receive `language-change-detected` webhook events, add it to `assistant.serverMessages`.  @default false | [optional]
**mip_opt_out** | Option<**bool**> | If set to true, this will add mip_opt_out=true as a query parameter of all API requests. See https://developers.deepgram.com/docs/the-deepgram-model-improvement-partnership-program#want-to-opt-out  This will only be used if you are using your own Deepgram API key.  @default false | [optional]
**numerals** | Option<**bool**> | If set to true, this will cause deepgram to convert spoken numbers to literal numerals. For example, \"my phone number is nine-seven-two...\" would become \"my phone number is 972...\"  @default false | [optional]
**keywords** | Option<**Vec<String>**> | These keywords are passed to the transcription model to help it pick up use-case specific words. Anything that may not be a common word, like your company name, should be added here. | [optional]
**keyterm** | Option<**Vec<String>**> | Keyterm Prompting allows you improve Keyword Recall Rate (KRR) for important keyterms or phrases up to 90%. | [optional]
**endpointing** | Option<**f64**> | This is the timeout after which Deepgram will send transcription on user silence. You can read in-depth documentation here: https://developers.deepgram.com/docs/endpointing.  Here are the most important bits: - Defaults to 10. This is recommended for most use cases to optimize for latency. - 10 can cause some missing transcriptions since because of the shorter context. This mostly happens for one-word utterances. For those uses cases, it's recommended to try 300. It will add a bit of latency but the quality and reliability of the experience will be better. - If neither 10 nor 300 work, contact support@vapi.ai and we'll find another solution.  @default 10 | [optional]
**fallback_plan** | Option<[**models::FallbackTranscriberPlan**](FallbackTranscriberPlan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


