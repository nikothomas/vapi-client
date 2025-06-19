# \ToolsApi

All URIs are relative to *https://api.vapi.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tool_controller_create**](ToolsApi.md#tool_controller_create) | **POST** /tool | Create Tool
[**tool_controller_find_all**](ToolsApi.md#tool_controller_find_all) | **GET** /tool | List Tools
[**tool_controller_find_one**](ToolsApi.md#tool_controller_find_one) | **GET** /tool/{id} | Get Tool
[**tool_controller_remove**](ToolsApi.md#tool_controller_remove) | **DELETE** /tool/{id} | Delete Tool
[**tool_controller_update**](ToolsApi.md#tool_controller_update) | **PATCH** /tool/{id} | Update Tool



## tool_controller_create

> models::ToolControllerFindAll200ResponseInner tool_controller_create(tool_controller_create_request)
Create Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tool_controller_create_request** | [**ToolControllerCreateRequest**](ToolControllerCreateRequest.md) |  | [required] |

### Return type

[**models::ToolControllerFindAll200ResponseInner**](ToolController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tool_controller_find_all

> Vec<models::ToolControllerFindAll200ResponseInner> tool_controller_find_all(limit, created_at_gt, created_at_lt, created_at_ge, created_at_le, updated_at_gt, updated_at_lt, updated_at_ge, updated_at_le)
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

[**Vec<models::ToolControllerFindAll200ResponseInner>**](ToolController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tool_controller_find_one

> models::ToolControllerFindAll200ResponseInner tool_controller_find_one(id)
Get Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToolControllerFindAll200ResponseInner**](ToolController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tool_controller_remove

> models::ToolControllerFindAll200ResponseInner tool_controller_remove(id)
Delete Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ToolControllerFindAll200ResponseInner**](ToolController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tool_controller_update

> models::ToolControllerFindAll200ResponseInner tool_controller_update(id, tool_controller_update_request)
Update Tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tool_controller_update_request** | [**ToolControllerUpdateRequest**](ToolControllerUpdateRequest.md) |  | [required] |

### Return type

[**models::ToolControllerFindAll200ResponseInner**](ToolController_findAll_200_response_inner.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

