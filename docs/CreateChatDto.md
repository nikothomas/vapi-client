# CreateChatDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant_id** | Option<**String**> | This is the assistant that will be used for the chat. To use an existing assistant, use `assistantId` instead. | [optional]
**assistant** | Option<[**models::CreateAssistantDto**](CreateAssistantDTO.md)> | This is the assistant that will be used for the chat. To use an existing assistant, use `assistantId` instead. | [optional]
**name** | Option<**String**> | This is the name of the chat. This is just for your own reference. | [optional]
**session_id** | Option<**String**> | This is the ID of the session that will be used for the chat. Mutually exclusive with previousChatId. | [optional]
**input** | [**models::CreateChatDtoInput**](CreateChatDTO_input.md) |  | 
**stream** | Option<**bool**> | This is a flag that determines whether the response should be streamed. When true, the response will be sent as chunks of text. | [optional][default to false]
**previous_chat_id** | Option<**String**> | This is the ID of the chat that will be used as context for the new chat. The messages from the previous chat will be used as context. Mutually exclusive with sessionId. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


