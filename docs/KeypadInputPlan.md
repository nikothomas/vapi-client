# KeypadInputPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | This keeps track of whether the user has enabled keypad input. By default, it is off.  @default false | [optional]
**timeout_seconds** | Option<**f64**> | This is the time in seconds to wait before processing the input. If the input is not received within this time, the input will be ignored. If set to \"off\", the input will be processed when the user enters a delimiter or immediately if no delimiter is used.  @default 2 | [optional]
**delimiters** | Option<**String**> | This is the delimiter(s) that will be used to process the input. Can be '#', '*', or an empty array. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


