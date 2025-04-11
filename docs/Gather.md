# Gather

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**output** | [**models::JsonSchema**](JsonSchema.md) |  | 
**confirm_content** | Option<**bool**> | This is whether or not the workflow should read back the gathered data to the user, and ask about its correctness. | [optional]
**hooks** | Option<[**Vec<models::Hook>**](Hook.md)> | This is a list of hooks for a task. Each hook is a list of tasks to run on a trigger (such as on start, on failure, etc). Only Say is supported for now. | [optional]
**max_retries** | Option<**f64**> | This is the number of times we should try to gather the information from the user before we failover to the fail path. An example of this would be a user refusing to give their phone number for privacy reasons, and then going down a different path on account of this | [optional]
**literal_template** | Option<**String**> | This is a liquid templating string. On the first call to Gather, the template will be filled out with variables from the context, and will be spoken verbatim to the user. An example would be \"Base on your zipcode, it looks like you could be in one of these counties: {{ counties | join: \", \" }}. Which one do you live in?\" | [optional]
**name** | **String** |  | 
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This is for metadata you want to store on the task. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


