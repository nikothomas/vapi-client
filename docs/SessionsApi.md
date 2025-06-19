# \SessionsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**session_controller_create**](SessionsApi.md#session_controller_create) | **POST** /session | Create Session
[**session_controller_find_all_paginated**](SessionsApi.md#session_controller_find_all_paginated) | **GET** /session | List Sessions
[**session_controller_find_one**](SessionsApi.md#session_controller_find_one) | **GET** /session/{id} | Get Session
[**session_controller_remove**](SessionsApi.md#session_controller_remove) | **DELETE** /session/{id} | Delete Session
[**session_controller_update**](SessionsApi.md#session_controller_update) | **PATCH** /session/{id} | Update Session



## session_controller_create

> models::Session session_controller_create(create_session_dto)
Create Session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_session_dto** | [**CreateSessionDto**](CreateSessionDto.md) |  | [required] |

### Return type

[**models::Session**](Session.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_controller_find_all_paginated

> models::SessionPaginatedResponse session_controller_find_all_paginated(name, assistant_id, workflow_id, page, sort_order, limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Sessions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | This is the name of the session to filter by. |  |
**assistant_id** | Option<**String**> | This is the ID of the assistant to filter sessions by. |  |
**workflow_id** | Option<**String**> | This is the ID of the workflow to filter sessions by. |  |
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

[**models::SessionPaginatedResponse**](SessionPaginatedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_controller_find_one

> models::Session session_controller_find_one(id)
Get Session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Session**](Session.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_controller_remove

> models::Session session_controller_remove(id)
Delete Session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Session**](Session.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_controller_update

> models::Session session_controller_update(id, update_session_dto)
Update Session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_session_dto** | [**UpdateSessionDto**](UpdateSessionDto.md) |  | [required] |

### Return type

[**models::Session**](Session.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

