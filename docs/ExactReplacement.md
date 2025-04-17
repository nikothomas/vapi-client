# ExactReplacement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | This is the exact replacement type. You can use this to replace a specific word or phrase with a different word or phrase.  Usage: - Replace \"hello\" with \"hi\": { type: 'exact', key: 'hello', value: 'hi' } - Replace \"good morning\" with \"good day\": { type: 'exact', key: 'good morning', value: 'good day' } - Replace a specific name: { type: 'exact', key: 'John Doe', value: 'Jane Smith' } - Replace an acronym: { type: 'exact', key: 'AI', value: 'Artificial Intelligence' } - Replace a company name with its phonetic pronunciation: { type: 'exact', key: 'Vapi', value: 'Vappy' } | 
**replace_all_enabled** | Option<**bool**> | This option let's you control whether to replace all instances of the key or only the first one. By default, it only replaces the first instance. Examples: - For { type: 'exact', key: 'hello', value: 'hi', replaceAllEnabled: false }. Before: \"hello world, hello universe\" | After: \"hi world, hello universe\" - For { type: 'exact', key: 'hello', value: 'hi', replaceAllEnabled: true }. Before: \"hello world, hello universe\" | After: \"hi world, hi universe\" @default false | [optional][default to false]
**key** | **String** | This is the key to replace. | 
**value** | **String** | This is the value that will replace the match. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


