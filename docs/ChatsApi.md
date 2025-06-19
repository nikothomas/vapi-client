# \ChatsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_controller_create_chat**](ChatsApi.md#chat_controller_create_chat) | **POST** /chat | Create Chat
[**chat_controller_create_open_ai_chat**](ChatsApi.md#chat_controller_create_open_ai_chat) | **POST** /chat/responses | Create Chat (OpenAI Compatible)
[**chat_controller_delete_chat**](ChatsApi.md#chat_controller_delete_chat) | **DELETE** /chat/{id} | Delete Chat
[**chat_controller_get_chat**](ChatsApi.md#chat_controller_get_chat) | **GET** /chat/{id} | Get Chat
[**chat_controller_list_chats**](ChatsApi.md#chat_controller_list_chats) | **GET** /chat | List Chats



## chat_controller_create_chat

> models::ChatControllerCreateChat200Response chat_controller_create_chat(create_chat_dto)
Create Chat

Creates a new chat. Requires at least one of: assistantId/assistant, sessionId, or previousChatId. Note: sessionId and previousChatId are mutually exclusive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_chat_dto** | [**CreateChatDto**](CreateChatDto.md) |  | [required] |

### Return type

[**models::ChatControllerCreateChat200Response**](ChatController_createChat_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_controller_create_open_ai_chat

> models::ChatControllerCreateOpenAiChat200Response chat_controller_create_open_ai_chat(open_ai_responses_request)
Create Chat (OpenAI Compatible)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_ai_responses_request** | [**OpenAiResponsesRequest**](OpenAiResponsesRequest.md) |  | [required] |

### Return type

[**models::ChatControllerCreateOpenAiChat200Response**](ChatController_createOpenAIChat_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_controller_delete_chat

> models::Chat chat_controller_delete_chat(id)
Delete Chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Chat**](Chat.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_controller_get_chat

> models::Chat chat_controller_get_chat(id)
Get Chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Chat**](Chat.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_controller_list_chats

> models::ChatPaginatedResponse chat_controller_list_chats(assistant_id, workflow_id, session_id, page, sort_order, limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Chats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | Option<**String**> | This is the unique identifier for the assistant that will be used for the chat. |  |
**workflow_id** | Option<**String**> | This is the unique identifier for the workflow that will be used for the chat. |  |
**session_id** | Option<**String**> | This is the unique identifier for the session that will be used for the chat. |  |
**page** | Option<**f64**> | This is the page number to return. Defaults to 1. |  |
**sort_order** | Option<**String**> | This is the sort order for pagination. Defaults to 'DESC'. |  |
**limit** | Option<**f64**> | This is the maximum number of items to return. Defaults to 100. |  |
**created_at_gt** | Option<**String**> | This will return items where the createdAt is greater than the specified value. |  |
**created_at_lt** | Option<**String**> | This will return items where the createdAt is less than the specified value. |  |
**created_at_ge** | Option<**String**> | This will return items where the createdAt is greater than or equal to the specified value. |  |
**created_at_le** | Option<**String**> | This will return items where the createdAt is less than or equal to the specified value. |  |
**updated_at_gt** | Option<**String**> | This will return items where the updatedAt is greater than the specified value. |  |
**updated_at_lt** | Option<**String**> | This will return items where the updatedAt is less than the specified value. |  |
**updated_at_ge** | Option<**String**> | This will return items where the updatedAt is greater than or equal to the specified value. |  |
**updated_at_le** | Option<**String**> | This will return items where the updatedAt is less than or equal to the specified value. |  |

### Return type

[**models::ChatPaginatedResponse**](ChatPaginatedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

