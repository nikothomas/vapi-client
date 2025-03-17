# TranscriptPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | This determines whether the transcript is stored in `call.artifact.transcript`. Defaults to true.  @default true | [optional]
**assistant_name** | Option<**String**> | This is the name of the assistant in the transcript. Defaults to 'AI'.  Usage: - If you want to change the name of the assistant in the transcript, set this. Example, here is what the transcript would look like with `assistantName` set to 'Buyer': ``` User: Hello, how are you? Buyer: I'm fine. User: Do you want to buy a car? Buyer: No. ```  @default 'AI' | [optional]
**user_name** | Option<**String**> | This is the name of the user in the transcript. Defaults to 'User'.  Usage: - If you want to change the name of the user in the transcript, set this. Example, here is what the transcript would look like with `userName` set to 'Seller': ``` Seller: Hello, how are you? AI: I'm fine. Seller: Do you want to buy a car? AI: No. ```  @default 'User' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


