# WorkflowOverrides

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**variable_values** | Option<[**serde_json::Value**](.md)> | These are values that will be used to replace the template variables in the workflow messages and other text-based fields. This uses LiquidJS syntax. https://liquidjs.com/tutorials/intro-to-liquid.html  So for example, `{{ name }}` will be replaced with the value of `name` in `variableValues`. `{{\"now\" | date: \"%b %d, %Y, %I:%M %p\", \"America/New_York\"}}` will be replaced with the current date and time in New York.  Some VAPI reserved defaults:  - *customer* - the customer object | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


