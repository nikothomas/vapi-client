# FormatPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | This determines whether the chunk is formatted before being sent to the voice provider. This helps with enunciation. This includes phone numbers, emails and addresses. Default `true`.  Usage: - To rely on the voice provider's formatting logic, set this to `false`.  If `voice.chunkPlan.enabled` is `false`, this is automatically `false` since there's no chunk to format.  @default true | [optional]
**number_to_digits_cutoff** | Option<**f64**> | This is the cutoff after which a number is converted to individual digits instead of being spoken as words.  Example: - If cutoff 2025, \"12345\" is converted to \"1 2 3 4 5\" while \"1200\" is converted to \"twelve hundred\".  Usage: - If your use case doesn't involve IDs like zip codes, set this to a high value. - If your use case involves IDs that are shorter than 5 digits, set this to a lower value.  @default 2025 | [optional]
**replacements** | Option<[**Vec<models::FormatPlanReplacementsInner>**](FormatPlan_replacements_inner.md)> | These are the custom replacements you can make to the chunk before it is sent to the voice provider.  Usage: - To replace a specific word or phrase with a different word or phrase, use the `ExactReplacement` type. Eg. `{ type: 'exact', key: 'hello', value: 'hi' }` - To replace a word or phrase that matches a pattern, use the `RegexReplacement` type. Eg. `{ type: 'regex', regex: '\\\\b[a-zA-Z]{5}\\\\b', value: 'hi' }`  @default [] | [optional]
**formatters_enabled** | Option<**Vec<String>**> | List of formatters to apply. If not provided, all default formatters will be applied. If provided, only the specified formatters will be applied. Note: Some essential formatters like angle bracket removal will always be applied. @default undefined | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


