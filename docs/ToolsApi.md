# \ToolsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tools_create**](ToolsApi.md#tools_create) | **POST** /tool | Create Tool
[**tools_delete**](ToolsApi.md#tools_delete) | **DELETE** /tool/{id} | Delete Tool
[**tools_get**](ToolsApi.md#tools_get) | **GET** /tool/{id} | Get Tool
[**tools_list**](ToolsApi.md#tools_list) | **GET** /tool | List Tools
[**tools_update**](ToolsApi.md#tools_update) | **PATCH** /tool/{id} | Update Tool



## tools_create

> models::ToolsCreateResponse tools_create(tools_create_request)
Create Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tools_create_request** | [**ToolsCreateRequest**](ToolsCreateRequest.md) |  | [required] |

### Return type

[**models::ToolsCreateResponse**](ToolsCreateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tools_delete

> models::ToolsDeleteResponse tools_delete(id)
Delete Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToolsDeleteResponse**](ToolsDeleteResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tools_get

> models::ToolsGetResponse tools_get(id)
Get Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToolsGetResponse**](ToolsGetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tools_list

> Vec<models::ToolsListResponseItem> tools_list(limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
List Tools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

[**Vec<models::ToolsListResponseItem>**](ToolsListResponseItem.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tools_update

> models::ToolsUpdateResponse tools_update(id, tools_update_request)
Update Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tools_update_request** | [**ToolsUpdateRequest**](ToolsUpdateRequest.md) |  | [required] |

### Return type

[**models::ToolsUpdateResponse**](ToolsUpdateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

