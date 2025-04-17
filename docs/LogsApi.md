# \LogsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logging_controller_logs_delete_query**](LogsApi.md#logging_controller_logs_delete_query) | **DELETE** /logs | Delete Logs
[**logging_controller_logs_query**](LogsApi.md#logging_controller_logs_query) | **GET** /logs | Get Logs



## logging_controller_logs_delete_query

> logging_controller_logs_delete_query(r#type, assistant_id, phone_number_id, customer_id, squad_id, call_id)
Delete Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | This is the type of the log. |  |
**assistant_id** | Option<**String**> |  |  |
**phone_number_id** | Option<**String**> | This is the ID of the phone number. |  |
**customer_id** | Option<**String**> | This is the ID of the customer. |  |
**squad_id** | Option<**String**> | This is the ID of the squad. |  |
**call_id** | Option<**String**> | This is the ID of the call. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logging_controller_logs_query

> models::LogsPaginatedResponse logging_controller_logs_query(r#type, webhook_type, assistant_id, phone_number_id, customer_id, squad_id, call_id, page, sort_order, limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
Get Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | This is the type of the log. |  |
**webhook_type** | Option<**String**> | This is the type of the webhook, given the log is from a webhook. |  |
**assistant_id** | Option<**String**> | This is the ID of the assistant. |  |
**phone_number_id** | Option<**String**> | This is the ID of the phone number. |  |
**customer_id** | Option<**String**> | This is the ID of the customer. |  |
**squad_id** | Option<**String**> | This is the ID of the squad. |  |
**call_id** | Option<**String**> | This is the ID of the call. |  |
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

[**models::LogsPaginatedResponse**](LogsPaginatedResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

