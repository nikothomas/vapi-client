# RegexReplacement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the regex replacement type. You can use this to replace a word or phrase that matches a pattern.  Usage: - Replace all numbers with \"some number\": { type: 'regex', regex: '\\\\d+', value: 'some number' } - Replace email addresses with \"[EMAIL]\": { type: 'regex', regex: '\\\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\\\.[A-Z|a-z]{2,}\\\\b', value: '[EMAIL]' } - Replace phone numbers with a formatted version: { type: 'regex', regex: '(\\\\d{3})(\\\\d{3})(\\\\d{4})', value: '($1) $2-$3' } - Replace all instances of \"color\" or \"colour\" with \"hue\": { type: 'regex', regex: 'colou?r', value: 'hue' } - Capitalize the first letter of every sentence: { type: 'regex', regex: '(?<=\\\\. |^)[a-z]', value: (match) => match.toUpperCase() } | 
**regex** | **String** | This is the regex pattern to replace.  Note: - This works by using the `string.replace` method in Node.JS. Eg. `\"hello there\".replace(/hello/g, \"hi\")` will return `\"hi there\"`.  Hot tip: - In JavaScript, escape `\\` when sending the regex pattern. Eg. `\"hello\\sthere\"` will be sent over the wire as `\"hellosthere\"`. Send `\"hello\\\\sthere\"` instead. | 
**options** | Option<[**Vec<models::RegexOption>**](RegexOption.md)> | These are the options for the regex replacement. Defaults to all disabled.  @default [] | [optional]
**value** | **String** | This is the value that will replace the match. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


