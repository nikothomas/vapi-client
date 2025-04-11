# BothCustomEndpointingRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant_regex** | **String** | This is the regex pattern to match the assistant's message.  Note: - This works by using the `RegExp.test` method in Node.JS. Eg. `/hello/.test(\"hello there\")` will return `true`.  Hot tip: - In JavaScript, escape `\\` when sending the regex pattern. Eg. `\"hello\\sthere\"` will be sent over the wire as `\"hellosthere\"`. Send `\"hello\\\\sthere\"` instead. - `RegExp.test` does substring matching, so `/cat/.test(\"I love cats\")` will return `true`. To do full string matching, send \"^cat$\". | 
**assistant_regex_options** | Option<[**Vec<models::RegexOption>**](RegexOption.md)> | These are the options for the assistant's message regex match. Defaults to all disabled.  @default [] | [optional]
**customer_regex** | **String** |  | 
**customer_regex_options** | Option<[**Vec<models::RegexOption>**](RegexOption.md)> | These are the options for the customer's message regex match. Defaults to all disabled.  @default [] | [optional]
**timeout_seconds** | **f64** | This is the endpointing timeout in seconds, if the rule is matched. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


