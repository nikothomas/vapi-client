# FormatPlanReplacementsItemOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**regex** | **String** | This is the regex pattern to replace.  Note: - This works by using the `string.replace` method in Node.JS. Eg. `\"hello there\".replace(/hello/g, \"hi\")` will return `\"hi there\"`.  Hot tip: - In JavaScript, escape `\\` when sending the regex pattern. Eg. `\"hello\\sthere\"` will be sent over the wire as `\"hellosthere\"`. Send `\"hello\\\\sthere\"` instead. | 
**options** | Option<[**Vec<models::RegexOption>**](RegexOption.md)> | These are the options for the regex replacement. Defaults to all disabled.  @default [] | [optional]
**value** | **String** | This is the value that will replace the match. | 
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


